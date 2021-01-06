export default class ImageCompositor {
  constructor(size, color) {
    this.canvas = document.createElement("canvas");
    this.canvas.width = this.canvas.height = size;
    this.canvas.style.display = "none";
    this.ctx = this.canvas.getContext("2d");
    this.ctx.fillStyle = color;
    this.image = new Image();
    this.image.style.display = "none";
  }

  composite(imageUrl, callback) {
    this.image.onload = () => {
      const size = this.canvas.width;
      this.ctx.fillRect(0, 0, size, size);
      this.ctx.drawImage(this.image, 0, 0, size, size);
      this.canvas.toBlob(callback);
    };
    this.image.src = imageUrl;
  }
}
