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

    cy.findByPlaceholderText("Character name...")
      .click()
      .clear()
      .type("character 2", { delay: 0 });
    cy.findByRole("button", { name: "Create Character" }).focus();
    cy.findByRole("button", { name: "Create Character" }).click();

    cy.findByPlaceholderText("Character name...")
      .click()
      .clear()
      .type("character 3", { delay: 0 });
    cy.findByRole("button", { name: "Create Character" }).focus();
    cy.findByRole("button", { name: "Create Character" }).click();

    cy.findByRole("button", { name: "Ready" }).click();
    // cy.findByText("Open Inventory").click();
    cy.findByText("Ready to explore").click();
  });
});
