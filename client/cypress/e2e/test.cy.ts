describe("a test test", () => {
  // cy.click()
  it("can enter a game", () => {
    cy.visit(`${Cypress.env("BASE_URL")}`, { failOnStatusCode: false });
    cy.findByRole("button", { name: "Create Game" }).click();
    cy.findByRole("button", { name: "Create Party" }).click();
    cy.findByRole("button", { name: "Create Character" }).click();
    cy.findByRole("button", { name: "Create Character" }).click();
    cy.findByRole("button", { name: "Create Character" }).click();
    cy.findByRole("button", { name: "Ready" }).click();
  });
});
