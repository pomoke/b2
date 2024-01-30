# Configuration

b2 can read configuration from EFI variables, regular file (UEFI only), and special partitions. On embedded platforms, b2 can be configured to use configuration embedded in image.

b2 configuration is in JSON or postcard. If no config found, b2 will use a fallback menu.

### EFI variables

UUID: `95f342d7-c48a-4799-8df5-6710597a7430`

### Fallback Menu

This menu can be used to manage power and exit (supported platforms only).
