<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  // Types
  enum NumberOfClicks {
    Single = "Single",
    Double = "Double"
  }

  enum ClickType {
    Left = "Left",
    Right = "Right"
  }

  interface Settings {
    click_speed_ms: number
  }

  let speed: number = 1;
  let numberOfClicks: NumberOfClicks = NumberOfClicks.Double
  let clickType: ClickType = ClickType.Left

  async function getSettings() {
    let result: Settings = await invoke("get_settings")

    speed = result.click_speed_ms;
  }

  async function setClickSpeed() {
    speed = await invoke("set_click_speed", { speed })
  }

  async function setNumberOfClicks() {
    numberOfClicks = await invoke("set_number_of_clicks", { numberOfClicks })
  }

  async function setClickType() {
    clickType = await invoke("set_click_type", { clickType })
  }

  getSettings()
</script>

<div>
  <div class="form-section">
    <label for="speed">Repeat speed in ms</label>
    <input type="number" name="speed" placeholder="Enter a value in ms" bind:value={speed} on:change={setClickSpeed} />
  </div>
  <div class="row">
    <div class="form-section">
      <label for="number-of-clicks">Number of clicks</label>
      <select id="number-of-clicks" name="number-of-clicks" bind:value={numberOfClicks} on:change={setNumberOfClicks}>
        <option value="Single">Single</option>
        <option value="Double">Double</option>
      </select>
    </div>
    <div class="form-section">
      <label for="number-of-clicks">Click type</label>
      <select id="click-type" name="click-type" bind:value={clickType} on:change={setClickType}>
        <option value="Left">Left</option>
        <option value="Right">Right</option>
      </select>
    </div>
  </div>
</div>
