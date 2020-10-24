use std::fmt;
use aes::{Aes128, cipher::{generic_array::GenericArray, {BlockCipher, NewBlockCipher}}};
use serde::{Deserializer, Deserialize};
use FormatError::*;

#[derive(Debug)]
pub enum FormatError { UnexpectedEof, BadMagic, BadAes, BadBase64, BadLength, BadMetadata }

type Result<T> = std::result::Result<T, FormatError>;

impl fmt::Display for FormatError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self) }
}

impl std::error::Error for FormatError {}

#[derive(Debug)]
pub enum MusicFormat { MP3, FLAC }

impl Default for MusicFormat {
  fn default() -> Self { MusicFormat::MP3 }
}

fn artist<'d, D>(d: D) -> std::result::Result<String, D::Error> where D: Deserializer<'d> {
  let list = <Vec<(&str, usize)>>::deserialize(d)?;
  let mut ret = String::new();
  let mut first = true;
  for (name, _) in list {
    if !first { ret.push_str(", "); }
    first = false;
    ret.push_str(name);
  }
  Ok(ret)
}

#[repr(C)]
#[derive(Debug, Default, Deserialize)]
pub struct Metadata {
  #[serde(rename = "musicName")] pub name: String,
  #[serde(deserialize_with = "artist")] pub artist: String,
  pub album: String,
  #[serde(skip)] pub image: Vec<u8>,
  #[serde(skip)] pub data: Vec<u8>,
  #[serde(skip)] pub format: MusicFormat,
}

extern "C" {
  #[allow(improper_ctypes)]
  fn add_tag(data: *const Metadata, raw: *const u8, len: usize);
}

#[no_mangle]
unsafe extern "C" fn alloc_vec(ptr: *mut (), size: usize) {
  let v = &mut *(ptr as *mut Vec<u8>);
  v.reserve(size);
  v.set_len(size);
}

fn aes_decrypt<'a>(key: &[u8; 16], buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
  let n = buf.len();
  if n == 0 || n % 16 != 0 { return Err(BadAes); }
  let aes = Aes128::new(GenericArray::from_slice(key));
  for i in 0..n / 16 {
    aes.decrypt_block(GenericArray::from_mut_slice(&mut buf[i * 16..(i + 1) * 16]));
  }
  let pad = buf[n - 1];
  Ok(if pad <= 16 { &mut buf[..n - pad as usize] } else { buf })
}

fn split(x: &mut [u8], n: usize) -> Result<(&mut [u8], &mut [u8])> {
  if n <= x.len() { Ok(x.split_at_mut(n)) } else { Err(UnexpectedEof) }
}

fn read_u32(x: &mut [u8]) -> Result<(u32, &mut [u8])> {
  split(x, 4).map(|(n, x)| (u32::from_le_bytes([n[0], n[1], n[2], n[3]]), x))
}

pub fn transform(ncm: &mut [u8]) -> Result<Metadata> {
  // check ncm file header magic
  let (magic1, ncm) = read_u32(ncm)?;
  let (magic2, ncm) = read_u32(ncm)?;
  if magic1 != 0x4e455443 || magic2 != 0x4d414446 { return Err(BadMagic); }
  let (_, ncm) = split(ncm, 2)?;

  // read key and build bey box
  let (n, ncm) = read_u32(ncm)?;
  let (buf, ncm) = split(ncm, n as usize)?;
  for x in buf.iter_mut() { *x ^= 0x64; }
  let buf = aes_decrypt(&[0x68, 0x7A, 0x48, 0x52, 0x41, 0x6D, 0x73, 0x6F, 0x35, 0x6B, 0x49, 0x6E, 0x62, 0x61, 0x78, 0x57], buf)?;
  let mut key_box = [0; 256];
  for (i, x) in key_box.iter_mut().enumerate() { *x = i as u8; }
  {
    // skip "neteasecloudmusic"
    let (_, key) = split(buf, 17)?;
    let mut last_pos = 0;
    let mut offset = 0;
    for i in 0..256 {
      let pos = (key_box[i] as usize + last_pos + key[offset] as usize) & 0xFF;
      let t = key_box[i];
      key_box[i] = key_box[pos];
      key_box[pos] = t;
      offset += 1;
      if offset >= key.len() { offset = 0; }
      last_pos = pos;
    }
  }

  // read meta data
  let (n, mut ncm) = read_u32(ncm)?;
  let mut base64_buf = Vec::new();
  let mut metadata = if n == 0 { Metadata::default() } else {
    let (buf, ncm1) = split(ncm, n as usize)?;
    ncm = ncm1;
    // skip "163 key(Don't modify):"
    let (_, buf) = split(buf, 22)?;
    for x in buf.iter_mut() { *x ^= 0x63; }
    base64::decode_config_buf(buf, base64::STANDARD, &mut base64_buf).map_err(|_| BadBase64)?;
    let buf = aes_decrypt(&[0x23, 0x31, 0x34, 0x6C, 0x6A, 0x6B, 0x5F, 0x21, 0x5C, 0x5D, 0x26, 0x30, 0x55, 0x3C, 0x27, 0x28], &mut base64_buf)?;
    // skip "music:"
    let (_, buf) = split(buf, 6)?;
    serde_json::from_slice::<Metadata>(buf).map_err(|_| BadMetadata)?
  };
  // skip crc32 & charset
  let (_, ncm) = split(ncm, 9)?;

  // read image data
  let (n, mut ncm) = read_u32(ncm)?;
  if n != 0 {
    let (buf, ncm1) = split(ncm, n as usize)?;
    ncm = ncm1;
    metadata.image = buf.into();
  }

  // read & transform music data
  for (i, x) in ncm.iter_mut().enumerate() {
    let j = (i + 1) & 0xFF;
    *x ^= key_box[(key_box[j] as usize + key_box[(key_box[j] as usize + j) & 0xFF] as usize) & 0xFF];
  }
  metadata.format = if ncm.get(..3).ok_or(UnexpectedEof)? == b"ID3" { MusicFormat::MP3 } else { MusicFormat::FLAC };

  // add tags to music via C++ implementation
  unsafe { add_tag(&mut metadata, ncm.as_ptr(), ncm.len()); }
  Ok(metadata)
}