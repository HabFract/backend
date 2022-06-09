
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../../utils';

export default (orchestrator: Orchestrator<any>) =>  {
  
  orchestrator.registerScenario("habit CRUD tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player]: Player[] = await s.players([config, config]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player]);

    const alice = alice_happ.cells.find(cell => cell.cellRole.includes('/habit_tree_engine.dna')) as Cell;
    const bob = bob_happ.cells.find(cell => cell.cellRole.includes('/habit_tree_engine.dna')) as Cell;

    const entryContents = {
  "name": "King any against",
  "timeframe": "Just my luck, no ice.  go, go, go, go, go! What do they got in there?",
  "habitMetadata": "So you two dig up, dig up dinosaurs? You're obsessed with the fat lady! You're obsessed with the fat lady!"
};

    // Alice creates a habit
    const create_output = await alice.call(
        "atomic_habits",
        "create_habit",
        entryContents
    );
    t.ok(create_output.headerHash);
    t.ok(create_output.entryHash);

    await sleep(200);
    
    // Bob gets the created habit
    const entry = await bob.call("atomic_habits", "get_habit", create_output.entryHash);
    t.deepEqual(entry, entryContents);
    
    
    // Alice updates the habit
    const update_output = await alice.call(
      "atomic_habits",
      "update_habit",
      {
        originalHeaderHash: create_output.headerHash,
        updatedHabit: {
          "name": "If know different",
  "timeframe": "Do you need a manager? God creates dinosaurs. I was part of something special.",
  "habitMetadata": "God creates Man. Man creates Dinosaurs.  God creates dinosaurs."
}
      }
    );
    t.ok(update_output.headerHash);
    t.ok(update_output.entryHash);
    await sleep(200);

      
    
    // Alice delete the habit
    await alice.call(
      "atomic_habits",
      "delete_habit",
      create_output.headerHash
    );
    await sleep(200);

    
    // Bob tries to get the deleted habit, but he doesn't get it because it has been deleted
    const deletedEntry = await bob.call("atomic_habits", "get_habit", create_output.entryHash);
    t.notOk(deletedEntry);
      
  });

}
