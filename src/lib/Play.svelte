<script lang="ts">
  export let onClick;
  export let message;
</script>

<div>
  <p>{message}</p>
  <button on:click|preventDefault={onClick}>
    <img src="/play.svg" alt="play" />
  </button>
</div>

<style scoped>
  img {
    width: 140px;
    height: 140px;
    border-radius: 50%;
  }
  img:hover {
    background-color: #b1264f;
  }
  button {
    background: none;
    border: none;
  }
  p {
    font-weight: bold;
  }
</style>
