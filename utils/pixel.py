from PIL import Image
import sys


def get_pixel_color(image_path, x, y):
    try:
        with Image.open(image_path) as img:
            img = img.convert("RGB")
            width, height = img.size

            if x < 0 or x >= width or y < 0 or y >= height:
                print("Coordinates are out of bounds.")
                return

            r, g, b = img.getpixel((x, y))
            print(f"RGB value at ({x}, {y}): ({r}, {g}, {b})")

    except FileNotFoundError:
        print(f"File not found: {image_path}")
    except Exception as e:
        print(f"An error occurred: {e}")


if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: python pixel.py <image_path> <x> <y>")
    else:
        image_path = sys.argv[1]
        x = int(sys.argv[2])
        y = int(sys.argv[3])
        get_pixel_color(image_path, x, y)
