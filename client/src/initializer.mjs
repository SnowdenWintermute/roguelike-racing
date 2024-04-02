export default function myInitializer() {
  const indicator = document.getElementById("loading-indicator");
  return {
    onStart: () => {
      indicator.innerHTML = `Fetching WebAssembly file...`;
    },
    onProgress: ({ current, total }) => {
      if (!total) {
        indicator.innerHTML = `Fetching WebAssembly file...`;
      } else {
        const percentLoaded = Math.round((current / total) * 100);
        if (percentLoaded !== 100) {
          indicator.innerHTML = `Fetching WebAssembly file ${percentLoaded}%`;
        } else {
          indicator.innerHTML = `Starting up...`;
        }
      }
    },
    onComplete: () => {
      indicator.classList.add("opacity-0");
      indicator.classList.add("pointer-events-none");
      setTimeout(() => {
        indicator.remove();
      }, 10050);
    },
    onSuccess: (_wasm) => {
      console.warn("Loading succeded!");
    },
    onFailure: (error) => {
      console.warn("Loading... failed!", error);
    },
  };
}
