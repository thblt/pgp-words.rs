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

`pgp-words` reads a fingerprint or any other hexadecimal sequence,
either from the command line or the standard input, and converts it
into words:

```
$ pgp-words d1c2 25e4 26c6 33dd 94d1  a7e8 c3f4 08aa 9b34 2488

d1c2 25e4 26c6 33dd 94d1 a7e8 c3f4 08aa 9b34 2488:
	stairway repellent bombast tradition
	bookshelf responsive chisel tambourine
	Pluto scavenger repay typewriter
	snowcap Virginia aimless pedigree
	puppy confidence bluebird maritime

gpg --fingerprint 0x33CDE511  | pgp-words

EA5B 81C5 6498 8C73 EE90  DEE9 BAF1 E072 33CD E511:
        Trojan exodus minnow resistor
        flytrap narrative offload hurricane
        tycoon millionaire tactics ultimate
        shadow vacancy tapeworm holiness
        chisel sandalwood topmost Babylon
```

 - When called with command-line arguments, it concatenates them and treats them as a single value.
 - When called from stdin, it treats each lins as a single value and silently ignores invalid lines.
 - For simplicity, spaces, tabs and newlines are ignored.
