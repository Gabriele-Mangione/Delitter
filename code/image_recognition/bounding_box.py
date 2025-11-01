from PIL import Image, ImageDraw, ImageFont

# NOTE: not used anywhere as of yet, perhaps useful for later / debugging

def draw_boxes_pil(
    image_path: str,
    boxes: list[tuple[int,int,int,int]],  # (x,y,w,h)
    labels: list[str] | None = None,
    colors: list[tuple[int,int,int]] | None = None,
    out_path: str = "annotated.jpg",
    thickness: int = 3,
    normalized: bool = False,  # set True if boxes in [0,1]
):
    img = Image.open(image_path).convert("RGB")
    W, H = img.size
    draw = ImageDraw.Draw(img)
    font = ImageFont.load_default()

    def to_px(x, y, w, h):
        if normalized:
            return int(x*W), int(y*H), int(w*W), int(h*H)
        return int(x), int(y), int(w), int(h)

    for i, b in enumerate(boxes):
        x, y, w, h = to_px(*b)
        # Clamp to image bounds
        x, y = max(0, x), max(0, y)
        w, h = max(1, min(W-x, w)), max(1, min(H-y, h))
        x2, y2 = x + w, y + h

        color = colors[i] if colors and i < len(colors) else (0, 255, 0)
        # Draw thick rectangle
        for t in range(thickness):
            draw.rectangle([x-t, y-t, x2+t, y2+t], outline=color)

        if labels and i < len(labels) and labels[i]:
            text = labels[i]
            tw, th = draw.textbbox((0,0), text, font=font)[2:]
            pad = 2
            draw.rectangle([x, y - th - 2*pad, x + tw + 2*pad, y], fill=color)
            draw.text((x + pad, y - th - pad), text, fill=(0,0,0), font=font)

    img.save(out_path)
    print(f"saved -> {out_path}")

if __name__ == "__main__":
    # Example
    boxes = [(50, 60, 120, 80), (300, 200, 150, 100)]
    labels = ["beverage_can", "snack_wrapper"]
    draw_boxes_pil("input.jpg", boxes, labels=labels, out_path="out_pil.jpg")
