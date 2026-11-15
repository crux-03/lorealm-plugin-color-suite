# Color Suite

Simple color-based conversion, picking, and operations.

## Processors
| Name              | Description                                                   |
|-------------------|---------------------------------------------------------------|
| random_hex        | Generates a random HEX color with an optional alpha component |
| random_rgb        | Generates a random RGB color with an optional alpha component |
| hex_to_rgb        | Converts a HEX color to an RGB color                          |
| rgb_to_hex        | Converts an RGB color to a HEX color                          |
| named_color       | Tries converting a string into a HEX color                    |
| random_color_name | Returns a random color name as a string                       |

## Developing

```sh
lorealm-plugin build
```

Then, in Lorealm, install it with "Add from source" and this folder.

## Releasing

```sh
lorealm-plugin stage 0.2.0   # sets the version, commits it and tags it
lorealm-plugin publish       # pushes the tag, which builds the release
```
