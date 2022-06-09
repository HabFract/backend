
import { LitElement, html } from 'lit';
import { state, customElement } from 'lit/decorators.js';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { contextProvided } from '@lit-labs/context';
import { appWebsocketContext, appInfoContext } from '../../../contexts';
import { Habit } from '../../../types/habit_tree_engine/atomic_habits';
import '@material/mwc-button';
import '@type-craft/title/create-title';
import '@type-craft/content/create-content';

@customElement('create-habit')
export class CreateHabit extends LitElement {

    @state()
  _name: string | undefined;

  @state()
  _timeframe: string | undefined;

  @state()
  _habitMetadata: string | undefined;

  isHabitValid() {
    return this._name && 
      this._timeframe && 
      this._habitMetadata;
  }

  @contextProvided({ context: appWebsocketContext })
  appWebsocket!: AppWebsocket;

  @contextProvided({ context: appInfoContext })
  appInfo!: InstalledAppInfo;

  async createHabit() {
    const cellData = this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'habit_tree_engine')!;

    const habit: Habit = {
      name: this._name!,
        timeframe: this._timeframe!,
        habitMetadata: this._habitMetadata!,
    };

    const { entryHash } = await this.appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: 'atomic_habits',
      fn_name: 'create_habit',
      payload: habit,
      provenance: cellData.cell_id[1]
    });

    this.dispatchEvent(new CustomEvent('habit-created', {
      composed: true,
      bubbles: true,
      detail: {
        entryHash
      }
    }));
  }

  render() {
    return html`
      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Habit</span>

        <create-title 
      
      @change=${(e: Event) => this._name = (e.target as any).value}
      style="margin-top: 16px"
    ></create-title>

        <create-content 
      
      @change=${(e: Event) => this._timeframe = (e.target as any).value}
      style="margin-top: 16px"
    ></create-content>

        <create-content 
      
      @change=${(e: Event) => this._habitMetadata = (e.target as any).value}
      style="margin-top: 16px"
    ></create-content>

        <mwc-button 
          label="Create Habit"
          .disabled=${!this.isHabitValid()}
          @click=${() => this.createHabit()}
        ></mwc-button>
    </div>`;
  }
}
