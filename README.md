# analogatique · web gallery for your photos

A beautiful static gallery generator (written in Rust 🦀) for anyone who wants to showcase their photos, whether film scans or digital images. It keeps the analog spirit with vintage aesthetics and Atkinson dithering while making it easy to create stunning web galleries.

**🎞️ See it in action [here](https://films.un1970ix.com).**

> [!WARNING]
> This project is currently under development and may introduce breaking changes.

## Installation

From [crates.io](https://crates.io/crates/analogatique):

```zsh
cargo install analogatique
```

Or download a prebuilt binary for your platform from the [latest release](https://github.com/un1970ix/analogatique/releases/latest). Linux, macOS, Windows, FreeBSD, and NetBSD are covered, and `.deb` and `.rpm` packages are published for x86_64 Linux.

Building from source requires Rust 1.88 or newer.

## Quick Start

1. Create a new directory for your gallery.

```zsh
mkdir films && cd films
```

2. Set up the required structure.

```zsh
analogatique init
```

3. Update `config.toml` with your gallery details and preferences.

4. Add your photos to the `photos/` directory.

5. Create `metadata.txt` by writing one JSON line for each photo or use `analogatique extract-metadata` to extract metadata from your photos.

```json
{"filename": "photo1.jpeg", "date": "15-06-2023", "name": "Sunset at the Beach", "camera": "Canon AE-1", "film": "FUJICOLOR C200"}
{"filename": "photo2.jpeg", "date": "20-06-2023", "camera": "Polaroid SX-70", "film": "B&W SX-70"}
```

6. Generate your gallery:

```zsh
analogatique generate
```

Your gallery will be created in the `public/` directory if you haven't changed the output directory.

## Commands

| Command            | Description                                            |
| ------------------ | ------------------------------------------------------ |
| `init`             | Create the gallery structure in the current directory. |
| `extract-metadata` | Read Exif data from `photos/` into `metadata.txt`.     |
| `generate`         | Build the static gallery.                              |
| `-v`, `--version`  | Print version information.                             |

## Configuration

Every section below is required in `config.toml`, even when its fields are left empty. `analogatique init` writes a complete file to start from.

| Key                       | Type    | Default    | Description                                                 |
| ------------------------- | ------- | ---------- | ----------------------------------------------------------- |
| `site.title`              | String  | Required   | Gallery title, used in the page title and header.           |
| `site.subtitle`           | String  | Required   | Shown beneath the title.                                    |
| `site.description`        | String  | Required   | Meta description, also shown in the footer.                 |
| `site.author`             | String  | Required   | Meta author tag.                                            |
| `dithering.enabled`       | Boolean | Required   | Apply Atkinson dithering to thumbnails.                     |
| `display.hide_filenames`  | Boolean | `false` \* | Hide filenames for photos with no `name` in their metadata. |
| `display.photos_per_page` | Integer | `0`        | Photos per page. `0` puts every photo on a single page.     |
| `output.path`             | String  | `"public"` | Directory the gallery is written to.                        |
| `footer.links`            | Array   | Required   | Footer links, each an inline table with `name` and `url`.   |

\* The default when the key is omitted is `false`, but `analogatique init` writes `hide_filenames = true`.

With pagination enabled, page one stays at `index.html` and later pages are written to `page/2/index.html`, `page/3/index.html`, and so on.

## Metadata Format

Each line in `metadata.txt` should be a JSON object with these fields:

| Field      | Description                                        | Required |
| ---------- | -------------------------------------------------- | -------- |
| `filename` | Name of the photo file.                            | Yes      |
| `date`     | Date the photo was taken. (DD-MM-YYYY)             | Yes      |
| `camera`   | Camera model used to take the photo.               | No       |
| `film`     | Name of the film used for the photo.               | No       |
| `lens`     | Lens used to capture the photo.                    | No       |
| `location` | Place where the photo was taken.                   | No       |
| `name`     | Custom title or name given to the photo.           | No       |
| `notes`    | Additional information or remarks about the photo. | No       |

## Supported Image Formats

- JPEG (.jpg, .jpeg)
- PNG (.png)
- TIFF (.tif, .tiff)

## Output Formats

- Thumbnails: WebP (You can enable optional Atkinson dithering.)
- Full Resolution: JPEG

## Customization

### Templates

The `templates/` directory contains:
- `index.html`
- `styles.css` files.

These use the [Tera](https://keats.github.io/tera/) templating engine. Customize them to match your style!

> [!IMPORTANT]
> Version 0.4.0 moved from Tera 1 to Tera 2. If you wrote a custom template against Tera 1, it may need updating: macros have been removed, several filters were renamed or moved to `tera-contrib`, and referencing an undefined variable now raises an error. See the [Tera migration guide](https://github.com/Keats/tera/blob/master/MIGRATION.md). The bundled templates already target Tera 2.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- I pay homage to Bill Atkinson for his Atkinson dithering algorithm and other work.
- I sincerely thank the [Trippy](https://github.com/fujiapple852/trippy) team for their help with the release workflow. The current workflow is based on theirs.
