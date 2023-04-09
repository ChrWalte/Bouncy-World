# The Bouncy World Engine

The Bouncy World Engine is a rust-lang application that allows for defined entities to bounce around a defined world. It starts as just a black box bouncing alone in a white world but can quickly grow into something quite creative by just using the [configuration system](https://github.com/ChrWalte/Bouncy-World#configuration-system) and the [world save system](https://github.com/ChrWalte/Bouncy-World#world-save-system). Check out some of the [examples](https://github.com/ChrWalte/Bouncy-World/tree/main/examples) listed in the Source code to get some inspiration.

## Installing The Bouncy World Engine

There are a few ways to install The Bouncy World Engine, either downloading the compiled binary from the GitHub Releases section or cloning the Source Code and compiling it yourself. Both are outlined below. This application was built to run standalone, out of the box, without configuration. This is where the lonely black box comes into play. After installing, the user can configure the look and feel of both the entities bouncing around and the world itself.

### Installing from GitHub Releases

Every release under the [GitHub Releases](https://github.com/ChrWalte/Bouncy-World/releases/) section contains the current source code and the built binary. Install the binary or executable to run this application on a Windows Intel Based x64 System.

## Running The Bouncy World Engine

Running The Bouncy World Engine will start a default, lonely black box bouncing in a white world. This default configuration can be customized extensively through the [configuration system](https://github.com/ChrWalte/Bouncy-World#configuration-system) and [world save system](https://github.com/ChrWalte/Bouncy-World#world-save-system).

### Running bouncy-world.exe

Running The Bouncy World Engine via the `bouncy-world.exe` is easy. Just run the executable and watch as a default bouncy world with a single entity bouncing around.

Running The Bouncy World Engine in the current directory:

```shell
.\\bouncy-world.exe
```

### Running from the Source Code

Running The Bouncy World Engine via the Source Code requires a few things:

The source code must be downloaded locally. This can be done via [git](https://git-scm.com/), a Source Code Version Control, or directly downloaded via GitHub as a [compressed ZIP file](https://github.com/ChrWalte/Bouncy-World/archive/refs/heads/main.zip).

Cloning The Bouncy World Engine Source Code to a local directory:

```shell
git clone https://github.com/ChrWalte/Bouncy-World.git
```

The [rust toolchain](https://www.rust-lang.org/tools/install) must be installed to build and run from the source code.
Once the toolchain is installed, use `cargo` to build and run The Bouncy World Engine.

```shell
cargo run --release
```

## Command Line Interface

The Bouncy World Engine has a Command Line Interface or CLI to create configurations and world saves.

### bouncy-would.exe CLI

To see a list of available commands, run the `help` command:

```shell
.\\bouncy-world.exe help
The Bouncy World Engine - v1.0.2

Usage: bouncy-world.exe <COMMAND>

Commands:
  new           generates a new default world save (in the known-universe folder)
  save          save a copy of the current world configuration (to the known-universe folder)
  config        generates a new default world configuration (if not already present)
  help          shows a list of all commands and their description
```

### cargo run CLI

These Command Line Arguments can be used via cargo as well:

```shell
cargo run --release -- help
The Bouncy World Engine - v1.0.2

Usage: bouncy-world.exe <COMMAND>

Commands:
  new           generates a new default world save (in the known-universe folder)
  save          save a copy of the current world configuration (to the known-universe folder)
  config        generates a new default world configuration (if not already present)
  help          shows a list of all commands and their description
```

## Configuration System

The Bouncy World Engine has a configuration system that allows for bouncy worlds to be loaded from a JSON or YAML file. these files need to be in the exact format for the version you are using.

Please refer to the examples on how to create your configuration files.

### Loading from a Configuration File

Loading from a configuration file in The Bouncy World Engine is simple. Just pass in the config path that yu would like to load.

```shell
.\\bouncy-world.exe .\\known-universe\\a-ransom-save\\config.json
.\\bouncy-world.exe .\\known-universe\\a-ransom-save\\config.yaml
.\\bouncy-world.exe .\\known-universe\\a-ransom-save\\config.yml
```

### Example JSON Configuration File

Here is an example JSON configuration file:

```jsonc
{
  // the version of The Bouncy World Engine that should be used for this configuration
  "bouncy_world_engine_version": "1.0.2",
  // for logging config and world information to the console (default: false)
  "is_debug_mode": false,

  // the world dimensions (default: width: 600, height: 400)
  "world_width": 600,
  "world_height": 400,
  // the world background hex color (default: ffffffff or White)
  "world_color": "ffffffff",

  // the type of world to use (default: Color)
  "world_type": "Color",
  // example of Image world type
  // user must specify image path and width and height of image
  // "world_type": {
  //   "Image": [
  //     "path-to-image.png",
  //     50,
  //     50
  //   ]
  // },

  // the number of entities to create (default: 1)
  "entity_count": 1,
  // the color of the entities (default: 000000ff or Black)
  "entity_color": "000000ff",
  // the velocity of entities, or how fast they are moving (default: 5)
  "entity_velocity": 5,

  // the type of entities to use (Default: Box: 50, 50)
  // user must specify the Box width and height
  "entity_type": {
    "Box": [
      50,
      50
    ]
  }
  // example of Ball entity type
  // user must specify radius of Ball
  // "entity_type": {
  //   "Ball": [
  //     25
  //   ]
  // },
  // example of Image entity type
  // user must specify image path and width and height of image
  // "entity_type": {
  //   "Image": [
  //     "path-to-image.png",
  //     50,
  //     50
  //   ]
  // }
}
```

### Example YAML (or YML) Configuration File

Here is an example YAML or YML configuration file:

```yaml
# the version of The Bouncy World Engine that should be used for this configuration
bouncy_world_engine_version: 1.0.2
# for logging config and world information to the console (default: false)
is_debug_mode: false

# the world dimensions (default: width: 600, height: 400)
world_width: 600
world_height: 400
# the world background hex color (default: ffffffff or White)
world_color: ffffffff
# the type of world to use (default: Color)
world_type: Color
# example of Image world type
# user must specify image path and width and height of image
# note: the world_color is ignored if an image is used
# world_type:
#   !Image
#   - "path-to-image.png"
#   - 50
#   - 50

# the number of entities to create (default: 1)
entity_count: 1
# the color of the entities (default: 000000ff or Black)
entity_color: 000000ff
# the velocity of entities, or how fast they are moving (default: 5)
entity_velocity: 5

# the type of entities to use (Default: Box: 50, 50)
# user must specify the Box width and height
entity_type:
  !Box
  - 50
  - 50
# example of Ball entity type
# user must specify radius of Ball
# entity_type:
#   !Ball
#   - 25
# example of Image entity type
# user must specify image path and width and height of image
# note: the entity_color is ignored if an image is used
# entity_type:
#   !Image
#   - "path-to-image.png"
#   - 50
#   - 50
```

## World Save System

### Loading from a World Save File

Loading from a world save file in The Bouncy World Engine is simple. Just pass in the bouncy-world path that yu would like to load.

```shell
.\\bouncy-world.exe .\\known-universe\\a-ransom-save\\a-random-save.bouncy-world
```

### Example Default bouncy-world Save File

```yaml
# date-time-stamp of when the world save was created
date_time_stamp: 0001-01-01T01.01.01
# the version of The Bouncy World Engine that should be used for this world save
bouncy_world_engine_version: 1.0.2

# the object world itself
# the user gets a lot more control over the world and its entities
world:
  # the world dimensions
  width: 600
  height: 400

  # the world background color
  background_rgba_hex: ffffffff

  # the world type
  world_type: Color

  # example image world type
  # note: the background_rgba_hex is ignored if using image world type
  # world_type:
  #   !Image
  #   - "path-to-image.png"
  #   - 50
  #   - 50

  # the entities bouncy around in the world
  entities:
    # each entity must be listed here
    # starting x and y positions of the entity
    - x_position: 230
      y_position: 215
      # x and y velocity of the entity
      x_velocity: 5
      y_velocity: -5
      # the color of the entity
      rgba_hex: 000000ff
      # the entity type
      entity_type:
        !Box
        - 50
        - 50

      # example ball entity type
      # entity_type:
      #   !Ball
      #   - 25
      # example image entity type
      # note: the rgba_hex is ignored if using image entity type
      # entity_type:
      #   !Image
      #   - "path-to-image.png"
      #   - 50
      #   - 50
```
