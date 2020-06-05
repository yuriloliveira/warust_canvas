window.onload = function () {
  const WIDTH = 600;
  const HEIGHT = 600;

  const image = document.getElementById("duck");
  const context = document.getElementById("canvas").getContext("2d");
  context.drawImage(image, 0, 0, WIDTH, HEIGHT);

  import("./pkg")
    .then((rustLib) => {
      document.getElementById("applyFilter").onclick = function () {
        rustLib.apply_filter(WIDTH, HEIGHT);
      };
    })
    .catch(console.error);
};
