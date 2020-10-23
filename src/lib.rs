use std::{fmt, io::*};
use byteorder::{LittleEndian, ReadBytesExt};
use aes::{Aes128, cipher::{generic_array::GenericArray, {BlockCipher, NewBlockCipher}}};
use serde::{Deserializer, Deserialize};

#[derive(Debug)]
pub enum FormatError { BadMagic, BadAes, BadBase64, BadLength, BadMetadata }

impl fmt::Display for FormatError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self) }
}

impl std::error::Error for FormatError {}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MusicFormat { MP3, FLAC }

impl Default for MusicFormat {
  fn default() -> Self { MusicFormat::MP3 }
}

fn artist<'d, D>(d: D) -> std::result::Result<Option<Vec<String>>, D::Error> where D: Deserializer<'d> {
  let list = <Option<Vec<(String, usize)>>>::deserialize(d)?;
  Ok(list.map(|x| x.into_iter().map(|x| x.0).collect()))
}

#[derive(Debug, Default, Deserialize)]
pub struct Metadata {
  #[serde(rename = "musicName")] pub name: Option<String>,
  #[serde(deserialize_with = "artist")] pub artist: Option<Vec<String>>,
  pub album: Option<String>,
  #[serde(skip)] pub image: Option<Vec<u8>>,
  #[serde(skip)] pub format: MusicFormat,
}

fn aes_decrypt(key: &[u8; 16], buf: &mut Vec<u8>) -> bool {
  let n = buf.len();
  if n == 0 || n % 16 != 0 { return false; }
  let aes = Aes128::new(GenericArray::from_slice(key));
  for i in 0..n / 16 {
    aes.decrypt_block(GenericArray::from_mut_slice(&mut buf[i * 16..(i + 1) * 16]));
  }
  let pad = buf[n - 1];
  if pad <= 16 { buf.resize(n - pad as usize, 0); }
  true
}

pub fn transform(mut ncm: impl ReadBytesExt + Seek, mut out: impl Write) -> Result<Metadata> {
  macro_rules! err {
    ($kind: ident) => { return Err(Error::new(ErrorKind::Other, FormatError::$kind)); };
  }
  // check ncm file header magic
  if ncm.read_u32::<LittleEndian>()? != 0x4e455443 || ncm.read_u32::<LittleEndian>()? != 0x4d414446 {
    err!(BadMagic);
  }
  ncm.seek(SeekFrom::Current(2))?;

  // read key and build bey box
  let n = ncm.read_u32::<LittleEndian>()? as usize;
  let mut buf = vec![0; n];
  ncm.read_exact(&mut buf)?;
  for x in &mut buf { *x ^= 0x64; }
  if !aes_decrypt(&[0x68, 0x7A, 0x48, 0x52, 0x41, 0x6D, 0x73, 0x6F, 0x35, 0x6B, 0x49, 0x6E, 0x62, 0x61, 0x78, 0x57], &mut buf) {
    err!(BadAes);
  }
  let mut key_box = [0; 256];
  for (i, x) in key_box.iter_mut().enumerate() { *x = i as u8; }
  {
    // skip "neteasecloudmusic"
    let key = if let Some(x) = buf.get(17..) { x } else { err!(BadLength) };
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
  let n = ncm.read_u32::<LittleEndian>()? as usize;
  let mut metadata = if n == 0 { Metadata::default() } else {
    // skip "163 key(Don't modify):"
    let n = if let Some(x) = n.checked_sub(22) { x } else { err!(BadLength) };
    ncm.seek(SeekFrom::Current(22))?;
    buf.resize(n, 0);
    ncm.read_exact(&mut buf)?;
    for x in &mut buf { *x ^= 0x63; }
    let mut buf = if let Ok(x) = base64::decode(&buf) { x } else { err!(BadBase64) };
    if !aes_decrypt(&[0x23, 0x31, 0x34, 0x6C, 0x6A, 0x6B, 0x5F, 0x21, 0x5C, 0x5D, 0x26, 0x30, 0x55, 0x3C, 0x27, 0x28], &mut buf) {
      err!(BadAes);
    }
    // skip "music:"
    let buf = if let Some(x) = buf.get(6..) { x } else { err!(BadLength) };
    serde_json::from_slice::<Metadata>(buf).unwrap();
    if let Ok(x) = serde_json::from_slice::<Metadata>(buf) { x } else { err!(BadMetadata) }
  };
  // skip crc32 & charset
  ncm.seek(SeekFrom::Current(9))?;

  // read image data
  let n = ncm.read_u32::<LittleEndian>()? as usize;
  metadata.image = if n == 0 { None } else {
    buf.resize(n, 0);
    ncm.read_exact(&mut buf)?;
    Some(buf)
  };

  // read & write music data
  let mut buf = vec![0; 4096];
  let mut first = true;
  loop {
    let n = ncm.read(&mut buf)?.min(buf.len());
    if n == 0 { break; }
    for (i, x) in buf[..n].iter_mut().enumerate() {
      let j = (i + 1) & 0xFF;
      *x ^= key_box[(key_box[j] as usize + key_box[(key_box[j] as usize + j) & 0xFF] as usize) & 0xFF];
    }
    out.write_all(&buf)?;
    if first {
      metadata.format = if buf[0] == 0x49 && buf[1] == 0x44 && buf[2] == 0x33 { MusicFormat::MP3 } else { MusicFormat::FLAC };
      first = false;
    }
  }
  Ok(metadata)
}