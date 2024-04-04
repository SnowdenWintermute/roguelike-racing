describe("a test test", () => {
  it("can enter a game", () => {
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

    cy.findByPlaceholderText("Character name...")
      .click()
      .clear()
      .type("Rebecca Chambers", { delay: 0 });
    cy.findByRole("button", { name: "Mage" }).click();
    cy.findByRole("button", { name: "Create Character" }).focus();
    cy.findByRole("button", { name: "Create Character" }).click();

    cy.findByPlaceholderText("Character name...")
      .click()
      .clear()
      .type("A Simple Job", { delay: 0 });
    cy.findByRole("button", { name: "Rogue" }).click();
    cy.findByRole("button", { name: "Create Character" }).focus();
    cy.findByRole("button", { name: "Create Character" }).click();

    cy.findByRole("button", { name: "Ready" }).click();
    // cy.findByText("Open Inventory").click();
    cy.findByText("Ready to explore").click({ force: true });

    // cy.findByText("Attack").click();
    // cy.findByText("Execute").click();
    // cy.wait(1200);
    // cy.findByText("Attack").click();
    // cy.findByText("Execute").click();
    // cy.wait(1200);
    // cy.findByText("Attack").click();
    // cy.findByText("Execute").click();
  });
});
