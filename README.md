# pgp-words

`pgp-words.rs` is a Rust implementation of the [PGP Word
List](https://en.wikipedia.org/wiki/PGP_word_list).  The word list was
designed to make it easy to convey sequences of bytes using only
voice, and is particularly suited for transmitting key fingerprints
and other similar sequences over the phone. Even in the same physical
place, it's easier to just read a bunch of words out loud than to
spell a sequence of hex digits (was that 3 times `F` or `3F`?)

Each word represents exactly one byte.  To avoid errors, there are
actually 512 words in the list (two per possible byte value) and words
are selected depending on the parity of the byte position.  For
example, `0x9e` is *quiver* if the byte position in the sequence is an
even number, *onlooker* if it is odd.

# Usage

The most basic usage is to just call `pgp-words` with the fingerprint
you want converted into words:

```
$ pgp-words d1c2 25e4 26c6 33dd 94d1  a7e8 c3f4 08aa 9b34 2488

d1c2 25e4 26c6 33dd 94d1 a7e8 c3f4 08aa 9b34 2488:
	stairway repellent bombast tradition
	bookshelf responsive chisel tambourine
	Pluto scavenger repay typewriter
	snowcap Virginia aimless pedigree
	puppy confidence bluebird maritime
```

It can also read from stdin, ignoring lines that aren't an hex string
(with `-s`)

```
gpg --fingerprint 0x33CDE511  | pgp-words -s

EA5B 81C5 6498 8C73 EE90  DEE9 BAF1 E072 33CD E511:
        Trojan exodus minnow resistor
        flytrap narrative offload hurricane
        tycoon millionaire tactics ultimate
        shadow vacancy tapeworm holiness
        chisel sandalwood topmost Babylon
```

It can also echo its input unmodified, annotating valid hex strings with words:

```
gpg --fingerprint 0x33CDE511  | pgp-words --passthrough --no-echo -s -p"  >>> " -w

pub   rsa4096 2017-05-04 [SC] [revoked: 2017-05-23]
      EA5B 81C5 6498 8C73 EE90  DEE9 BAF1 E072 33CD E511
  >>> Trojan exodus minnow resistor flytrap narrative offload hurricane tycoon millionaire
  >>> tactics ultimate shadow vacancy tapeworm holiness chisel sandalwood topmost Babylon
uid           [ revoked] Thibault Polge <thibault@thb.lt>
```
