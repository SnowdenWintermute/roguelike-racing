const io = require("socket.io")(9090);
console.log("started cypress sync socket.io server");
let lastCheckpoint;

io.on("connection", (socket) => {
  console.log("a cypress instance connected");

  if (lastCheckpoint) {
    console.log('sending the last checkpoint "%s"', lastCheckpoint);
    socket.emit("checkpoint", lastCheckpoint);
  }

  socket.on("disconnect", () => {
    console.log("disconnected");
  });

  socket.on("checkpoint", (name) => {
    console.log('chat checkpoint: "%s"', name);
    lastCheckpoint = name;
    io.emit("checkpoint", name);
  });
});
