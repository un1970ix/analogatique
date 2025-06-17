# analogatique · web gallery for you photos

A beautiful static gallery generator (written in Rust 🦀) for anyone who wants to showcase their photos, whether film scans or digital images. It keeps the analog spirit with vintage aesthetics and Atkinson dithering while making it easy to create stunning web galleries.

> [!WARNING]
> This project is currently under development and may introduce breaking changes.

## Quick Start

1. Create a new directory for your gallery.

```zsh
mkdir films && cd films
```

2. Set up the required structure.

```zsh
mkdir photos templates
```

3. Create `config.toml`.

```toml
[site]
title = "Films"
subtitle = "Took on 35mm."
description = "A collection of analog moments."
author = "Nicéphore Niépce"

[dithering]
enabled = true

[display]
hide_filenames = true

[footer]
links = [
    { url = "", name = "" },
    { url = "", name = "" },
]
```

4. Create `metadata.txt` by writing one JSON line for each photo.

```json
{"filename": "photo1.jpeg", "date": "15-06-2023", "name": "Sunset at the Beach", "camera": "Canon AE-1", "film": "FUJICOLOR C200"}
{"filename": "photo2.jpeg", "date": "20-06-2023", "camera": "Polaroid SX-70", "film": "B&W SX-70"}
```

5. Copy the default templates.

```zsh
curl -L https://raw.githubusercontent.com/un1970ix/analogatique/master/templates/index.html > templates/index.html
curl -L https://raw.githubusercontent.com/un1970ix/analogatique/master/templates/styles.css > templates/styles.css
```

6. Add your photos to the `photos/` directory.

7. Generate your gallery:

```zsh
analogatique
```

Your gallery will be created in the `public/` directory!

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

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- I pay homage to Bill Atkinson for his Atkinson dithering algorithm and other work.
- I sincerely thank the [Trippy](https://github.com/fujiapple852/trippy) team for their help with the release workflow. The current workflow is based on theirs.
