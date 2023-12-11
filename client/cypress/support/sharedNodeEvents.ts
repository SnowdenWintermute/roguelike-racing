import io from "socket.io-client";

export default function sharedNodeEvents(
  on: Cypress.PluginEvents,
  _: Cypress.PluginConfigOptions
) {
  const socket = io("http://localhost:9090");
  let checkpointName: String;
  socket.on("checkpoint", (name: String) => {
    console.log("current checkpoint %s", name);
    checkpointName = name;
  });

  on("task", {
    // tasks for syncing multiple Cypress instances together
    checkpoint(name) {
      console.log('emitting checkpoint name "%s"', name);
      socket.emit("checkpoint", name);

      return null;
    },
    waitForCheckpoint(name) {
      console.log('waiting for checkpoint "%s"', name);

      // TODO: set maximum waiting time
      return new Promise((resolve) => {
        const i = setInterval(() => {
          console.log('checking, current checkpoint "%s"', checkpointName);
          if (checkpointName === name) {
            console.log('reached checkpoint "%s"', name);
            clearInterval(i);
            resolve(name);
          }
        }, 1000);
      });
    },
  });
}
