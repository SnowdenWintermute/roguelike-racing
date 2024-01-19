// store the results in a queue
// pass the first result to the entity and have them animate
//   -- approach
//   -- swing to contact
//   -- follow through swing
//     -- reduce hp
//     -- animate hit recovery
//     -- floating numbers
//   -- query queue for next result
//   -- swing to contact
//     -- reduce hp
//     -- animate hit recovery
//     -- floating numbers
//   -- follow through swing
//   -- return to spot
//   -- pass turn
//
//   Entities have:
//   current action result processing (if any)
//   when an action result is passed, start animating
//    - push to a queue of animations (move, swing to hit [damage here], follow through, recover, return)
//    - animations have an on_finish which can trigger animations on other entities, interrupting
//    their current hit recovery animation if any (getting hit before hit recovery animation finishes). Trigger
//    the on_finish for that animation (floating numbers, combat log entry)
//    - if take damage while in an action animation, just reduce the hp and show the floating
//    numbers
//    - if die while in an action animation, show floating numbers and, play the death animation in place
//    - entities can not select (or execute) a new action until their animaton queues are finished
//
