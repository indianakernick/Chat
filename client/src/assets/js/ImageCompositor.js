export default class ImageCompositor {
  constructor(size, color) {
    this.canvas = document.createElement("canvas");
    this.canvas.width = this.canvas.height = size;
    this.canvas.style.display = "none";
    document.body.appendChild(this.canvas);
    this.ctx = this.canvas.getContext("2d");
    this.ctx.fillStyle = color;
  }

  composite(imageUrl, callback) {
    if (imageUrl.length === 0) return;
    const image = new Image();
    image.style.display = "none";
    image.referrerPolicy = "no-referrer";
    image.crossOrigin = "anonymous";
    image.onload = () => {
      const size = this.canvas.width;
      this.ctx.fillRect(0, 0, size, size);
      this.ctx.drawImage(image, 0, 0, size, size);
      this.canvas.toBlob(callback);
    };
    image.src = imageUrl;
  }
}

export const comp64 = new ImageCompositor(64, "#e9ecef"); // $group-item-back
export const comp48 = new ImageCompositor(48, "#e9ecef"); // $user-picture-back
export const comp32 = new ImageCompositor(32, "#e9ecef"); // $user-picture-back
