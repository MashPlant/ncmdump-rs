#include <cstdio>
#include <cstring>
#include <taglib/attachedpictureframe.h>
#include <taglib/fileref.h>
#include <taglib/flacfile.h>
#include <taglib/id3v2tag.h>
#include <taglib/mpegfile.h>
#include <taglib/tbytevectorstream.h>

struct Vec {
  char *ptr;
  size_t cap;
  size_t len;
};

struct Metadata {
  Vec name, artist, album, image, data;
  unsigned char format;
};

static unsigned char PNG[8] = {0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A};

extern "C" {
void alloc_vec(void *, size_t);

void add_tag(Metadata &data, const char *raw, size_t len) {
  using namespace TagLib;
  ByteVectorStream stream(ByteVector(raw, len));
  File *f;
  ID3v2::Tag *tag;
  if (data.format == 0) { // MP3
    MPEG::File *f1 = new MPEG::File(&stream, ID3v2::FrameFactory::instance());
    f = f1, tag = f1->ID3v2Tag(true);
  } else { // FLAC
    FLAC::File *f1 = new FLAC::File(&stream, ID3v2::FrameFactory::instance());
    f = f1, tag = f1->ID3v2Tag(true);
  }
  if (data.image.len != 0) {
    ID3v2::AttachedPictureFrame *frame = new TagLib::ID3v2::AttachedPictureFrame;
    frame->setMimeType(memcmp(data.image.ptr, PNG, 8) == 0 ? "image/png" : "image/jpeg");
    frame->setPicture(ByteVector(data.image.ptr, data.image.len));
    tag->addFrame(frame);
  }
  if (data.name.len != 0) { tag->setTitle(String(ByteVector(data.name.ptr, data.name.len), String::UTF8)); }
  if (data.artist.len != 0) { tag->setTitle(String(ByteVector(data.artist.ptr, data.artist.len), String::UTF8)); }
  if (data.album.len != 0) { tag->setAlbum(String(ByteVector(data.album.ptr, data.album.len), String::UTF8)); }
  f->save();
  ByteVector *result = stream.data();
  size_t size = result->size();
  alloc_vec(&data.data, size);
  memcpy(data.data.ptr, result->data(), size);
  delete f;
}
}