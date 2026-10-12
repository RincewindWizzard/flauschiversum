"""Erzeugt assets/badge.png: Wasserzeichen im Farbschema „Flauschi-Nebel“ (800 × 133, RGBA).

Aufruf (aus dem Projektverzeichnis): python3 openspec/changes/post-ansicht-ueberarbeiten/assets/make_badge.py
Farben entsprechen content/style/theme.css (hell): Fläche --accent-soft, Schrift --acolor, Rand --accent.
"""
from PIL import Image, ImageDraw, ImageFont

W, H, S = 800, 133, 4  # Zielgröße, Überabtastung
FILL = (0xE9, 0xE4, 0xFA, 210)    # --accent-soft, halbtransparent
INK = (0x1D, 0x19, 0x30, 240)     # --acolor
EDGE = (0x6D, 0x4F, 0xE0, 200)    # --accent
TEXT = "Flauschiversum"
TARGET_TEXT_WIDTH = 706 * S       # wie das bisherige Badge

font_path = "content/static/fonts/Lobster.otf"
size = 100 * S
font = ImageFont.truetype(font_path, size)
width = font.getbbox(TEXT)[2] - font.getbbox(TEXT)[0]
font = ImageFont.truetype(font_path, int(size * TARGET_TEXT_WIDTH / width))

img = Image.new("RGBA", (W * S, H * S), (0, 0, 0, 0))
d = ImageDraw.Draw(img)
r = 58 * S
d.rounded_rectangle((0, 0, W * S - 1, H * S - 1), radius=r, fill=EDGE)
b = 4 * S  # Randstärke
d.rounded_rectangle((b, b, W * S - 1 - b, H * S - 1 - b), radius=r - b, fill=(0, 0, 0, 0))
# Fläche innerhalb des Rands (ersetzt statt zu überlagern, damit die Alphawerte stimmen)
fill = Image.new("RGBA", img.size, (0, 0, 0, 0))
ImageDraw.Draw(fill).rounded_rectangle((b, b, W * S - 1 - b, H * S - 1 - b), radius=r - b, fill=FILL)
img = Image.alpha_composite(fill, img) if False else Image.composite(fill, img, fill.split()[3])
bbox = font.getbbox(TEXT)
x = (W * S - (bbox[2] - bbox[0])) // 2 - bbox[0]
y = (H * S - (bbox[3] - bbox[1])) // 2 - bbox[1]
text = Image.new("RGBA", img.size, (0, 0, 0, 0))
ImageDraw.Draw(text).text((x, y), TEXT, font=font, fill=INK)
img = Image.composite(text, img, text.split()[3])
img.resize((W, H), Image.LANCZOS).save("assets/badge.png", optimize=True)
