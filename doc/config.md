# Configuration

b2 can read configuration from EFI variables, regular files, or a special partition. 

b2 configuration is in JSON or postcard (TOML support in progress). If no valid config was found, b2 will use a fallback menu.

## EFI variables

b2 variables are under vendor UUID `95f342d7-c48a-4799-8df5-6710597a7430`.

* `Config`: Configuration to use, if no config file found. Could be in JSON or postcard format. 
* `Logs`: Log produced when running. Can be inspected later.

### systemd Boot Loader Interface
This interface is useful for boot analysis and on systemd-based Linux distributions, under UUID `a67b082-0a4c-41cf-b6c7-440b29bb8c4f`.
Check <https://systemd.io/BOOT_LOADER_INTERFACE/> for detail, but not all variables will be implemented.

## Fallback Menu

This menu can be used to manage power and exit (on supported platforms only), in case of non-existant or broken config.

## i18n

Todo for now. This requires external Unicode font or pre-generated prompts in image, and both require much effort.