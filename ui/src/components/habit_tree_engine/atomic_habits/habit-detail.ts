
import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { contextProvided } from '@lit-labs/context';
import { appInfoContext, appWebsocketContext } from '../../../contexts';
import { Habit } from '../../../types/habit_tree_engine/atomic_habits';
import '@material/mwc-circular-progress';
import '@type-craft/title/title-detail';
import '@type-craft/content/content-detail';

@customElement('habit-detail')
export class HabitDetail extends LitElement {
  @property()
  entryHash!: string;

  @state()
  _habit: Habit | undefined;

  @contextProvided({ context: appWebsocketContext })
  appWebsocket!: AppWebsocket;

  @contextProvided({ context: appInfoContext })
  appInfo!: InstalledAppInfo;

  async firstUpdated() {
    const cellData = this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'habit_tree_engine')!;

    this._habit = await this.appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: 'atomic_habits',
      fn_name: 'get_habit',
      payload: this.entryHash,
      provenance: cellData.cell_id[1]
    });
  }

  render() {
    if (!this._habit) {
      return html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`;
    }

    return html`
      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Habit</span>

        
    <title-detail
    
    .value=${this._habit.name}
      style="margin-top: 16px"
    ></title-detail>

        
    <content-detail
    
    .value=${this._habit.timeframe}
      style="margin-top: 16px"
    ></content-detail>

        
    <content-detail
    
    .value=${this._habit.habitMetadata}
      style="margin-top: 16px"
    ></content-detail>

      </div>
    `;
  }
}
