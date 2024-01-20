describe("a test test", () => {
  it("is a second user", () => {
    cy.visit(`${Cypress.env("BASE_URL")}`, { failOnStatusCode: false });
    //
    cy.task("waitForCheckpoint", "game created");
    cy.findByRole("button", { name: "Refresh List" }).click();
    cy.findByRole("button", { name: "Join" }).click();
    cy.findByRole("button", { name: "Join Party" }).click();
    cy.findByPlaceholderText("Character name...").click().type("Lt. Tsurumi", {
      delay: 0,
    });
    cy.findByRole("button", { name: "Create Character" }).focus();
    cy.findByRole("button", { name: "Create Character" }).click();
    cy.task("checkpoint", "second player character created");
    cy.findByRole("button", { name: "Ready" }).click();
    cy.findByText("Ready to explore").click();

    cy.task("waitForCheckpoint", "first attack executed");

    cy.findByText("Attack").click();
    cy.findByText("Execute").click();

    // cy.findAllByText("Take").first().click({ force: true });
    // cy.findAllByText("Take").first().click({ force: true });
    // cy.findAllByText("Take").first().click({ force: true });
  });
});
