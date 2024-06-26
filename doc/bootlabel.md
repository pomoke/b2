# Bootlabel

Bootlabel is a simple on-disk format for bootloader.

This is a draft.

## On-Disk Layout
All fields are in little endian.

### Bootlabel Header

signature: uint64 of "bootl \x7f bl" (0x6c627f6c746f6f62)
checksum: uint32
version: uint16 
header_size: uint16
config: uint16
id: uint64
num_slots: uint32
default_owner: uint16
name: char[64]
// Align to 128 bytes.
slots: slot[]

slot:
type: uint32
slot_id: uint16
sub_id: uint16
owner_id: uint16
flag: uint32
offset: uint32 
length: uint32
name: char[32]

### Slot
Slot contains data, program or configuration. 


### Types

### Owner
Owner is a random generated unsigned integer generated by OS installation to identify which items belong to a specific OS.
The owner id of 0 denotes this a global slot.

#### Global Types

#### Slot Types
The slot type can be splited into two parts, one of the higher 12 bits, and lower 20 bits.
The high 12 bits is used to denote platform and/or architectures where applicable.

* 0: Unused
* 0x100: Global Configuration
* 0x101: Environment Block
* 0x102: Parameter Block
* 0x103: Text
* 0x104: i18n Data Block
* 0x105: Font Block
* 0x110: Local Config
* 0x111: Bootconfig

* 0x180: Random Seed
* 0x200: Linux Kernel
* 0x201: Linux Initrd
* 0x210: Multiboot Kernel
* 0x211: Multiboot Module
* 0x220: Chainload

* 0x1___: Private use

### Flags

#### Global flags

#### Slot flags
* FAILBACK_DEFAULT: The default boot target when failed to read config.


### Auto-detection Convention
// TODO