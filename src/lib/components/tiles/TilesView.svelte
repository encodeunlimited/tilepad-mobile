<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { FolderModel } from "$lib/api/types/folders";

  import { fly } from "svelte/transition";
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";
  import DigitalClock from "$lib/components/DigitalClock.svelte";

  type Props = {
    tiles: TileModel[];
    folder: FolderModel;
    onClick: (tileId: string) => void;
  };

  const { tiles, folder, onClick }: Props = $props();
</script>

<div class="container">
  <header class="clock-header">
    <DigitalClock />
  </header>

  <div class="content-area">
    {#key folder.id}
      <div
        class="grid-wrapper"
        in:fly={{ x: -50, duration: 300, opacity: 0 }}
        out:fly={{ x: 50, duration: 300, opacity: 0 }}
      >
        <TileGrid
          {tiles}
          rows={folder.config.rows}
          columns={folder.config.columns}
          onClickTile={(tile) => {
            onClick(tile.id);
          }}
        />
      </div>
    {/key}
  </div>
</div>

<style>
  .container {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .clock-header {
    flex-shrink: 0;
    padding-top: 0.25rem;
  }

  .content-area {
    position: relative;
    flex: 1;
    width: 100%;
    overflow: hidden;
  }

  .grid-wrapper {
    position: absolute;
    display: flex;
    flex: auto;
    flex-flow: column;

    height: 100%;
    width: 100%;

    overflow: hidden;
    padding: 0.5rem 1rem 1rem 1rem;
  }
</style>
