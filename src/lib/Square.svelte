<script lang="ts">
  export let bgcolor;
  export let onClick;
</script>

<button style:background-color={bgcolor} on:click|preventDefault={onClick}
></button>

<style scoped>
  button {
    width: 16%;
    height: 16%;
    border: none;
    border-radius: 8px;
  }
</style>
