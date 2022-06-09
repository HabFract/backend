import { Orchestrator } from "@holochain/tryorama";

import atomic_habits_habit from './habit_tree_engine/atomic_habits/habit';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
atomic_habits_habit(orchestrator);
orchestrator.run();



