<script lang="ts">
  import Square from "./Square.svelte";
  import Play from "./Play.svelte";
  import { invoke } from "@tauri-apps/api/core";

  const correct_audio = new Audio("/correct.mp3");
  const error_audio = new Audio("/error.mp3");

  let game = {};
  let timeOut = false;
  let game_on = false;
  let timeOutID = [];
  let user_response = false;
  let message = "Ready to Play";

  $: if (user_response) {
    for (let id in timeOutID) {
      clearTimeout(timeOutID[id]);
    }
    timeOutID = [];
    timeOut = false;
    user_response = false;
  }

  function startTimer() {
    timeOutID.push(
      setTimeout(() => {
        timeOut = true;
        message = `Time over! You got ${game.level} 🪙`;
        error_audio.play();
      }, 3000),
    );
  }

  async function newGame() {
    game_on = true;
    timeOut = false;
    game = await invoke("new_game");
    startTimer();
  }

  async function nextLevel() {
    user_response = true;
    correct_audio.play();
    game = await invoke("next_level");
    startTimer();
  }

  function gameOver() {
    message = `Wrong! You got ${game.level} 🪙`;
    user_response = true;
    game_on = false;
    error_audio.play();
  }
</script>

<div class="main">
  {#if game_on}
    {#await newGame()}
      <p>Loading</p>
    {:then}
      {#if !timeOut}
        <div class="subdiv">
          <p style="color:#ffd700;font-size:20px;">🪙 {game.level}</p>
          <div class="game">
            {#each [0, 1, 2, 3, 4, 5] as row}
              {#each [0, 1, 2, 3, 4, 5] as col}
                {#if game.square[0] == row && game.square[1] == col}
                  <Square
                    bgcolor={`rgb(${game.true_color[0]},${game.true_color[1]},${game.true_color[2]})`}
                    onClick={nextLevel}
                  />
                {:else}
                  <Square
                    bgcolor={`rgb(${game.color[0]},${game.color[1]},${game.color[2]})`}
                    onClick={gameOver}
                  />
                {/if}
              {/each}
            {/each}
          </div>
        </div>
      {:else}
        <div class="subdiv">
          <Play onClick={newGame} {message} />
        </div>
      {/if}
    {:catch error}
      <p style="color: red">{error.message}</p>
    {/await}
  {:else}
    <Play onClick={newGame} {message} />
  {/if}
</div>

<style scoped>
  .game {
    width: min(80vw, 60vh);
    height: min(80vw, 60vh);
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    flex-flow: wrap;
    gap: 1px;
  }
  .main {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    justify-content: center;
    gap: 100px;
  }
  .subdiv {
    display: flex;
    flex-direction: column;
  }
</style>
