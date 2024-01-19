describe("a test test", () => {
  it("is the first user", () => {
    cy.visit(`${Cypress.env("BASE_URL")}`, { failOnStatusCode: false });
    cy.findByRole("button", { name: "Create Game" }).click();
    cy.findByRole("button", { name: "Create Party" }).click();

    cy.findByPlaceholderText("Character name...")
      .click()
      .type("KingReaverKirito", {
        delay: 0,
      });
    cy.findByRole("button", { name: "Create Character" }).focus();
    cy.findByRole("button", { name: "Create Character" }).click();

    // cy.findByPlaceholderText("Character name...")
    //   .click()
    //   .clear()
    //   .type("R. Chambers", { delay: 0 });
    // cy.findByRole("button", { name: "Create Character" }).focus();
    // cy.findByRole("button", { name: "Create Character" }).click();

    cy.task("checkpoint", "game created");
    cy.task("waitForCheckpoint", "second player character created");

    cy.findByRole("button", { name: "Ready" }).click();
    // cy.findByText("Open Inventory").click();
    cy.findByText("Ready to explore").click();
    cy.findByText("Attack").click();
    cy.findByText("Execute").click();
    cy.task("checkpoint", "first attack executed");
  });
});
