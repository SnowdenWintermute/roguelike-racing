import { defineConfig } from "cypress";
import sharedNodeEvents from "./cypress/support/sharedNodeEvents";

module.exports = defineConfig({
  e2e: {
    setupNodeEvents(on, config) {
      // implement node event listeners here
      sharedNodeEvents(on, config);
    },
    fixturesFolder: false,
    baseUrl: "http://localhost:8080",
    specPattern: "**/first-user.cy.ts",
    // viewportWidth: 400,
    // viewportHeight: 400,
    defaultCommandTimeout: 15000,
    videosFolder: "cypress/videos-pair/first",
    screenshotsFolder: "cypress/screenshots-pair/first",
    // $schema: "https://on.cypress.io/cypress.schema.json",
  },
});
