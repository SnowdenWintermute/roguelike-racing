describe("a test test", () => {
  // cy.click()
  it("can enter a game", () => {
    cy.visit(`${Cypress.env("BASE_URL")}`, { failOnStatusCode: false });
  });
});
// describe('My First Test', () => {
//   it('Does not do much!', () => {
//     expect(true).to.equal(true)
//   })
// })
