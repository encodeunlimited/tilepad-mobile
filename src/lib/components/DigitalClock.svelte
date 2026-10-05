<script lang="ts">
  import { onMount } from "svelte";

  let timeString = $state("");
  let ampmString = $state("");
  let dateString = $state("");

  function updateClock() {
    const now = new Date();

    // Format hours, minutes, seconds
    let hours = now.getHours();
    const minutes = String(now.getMinutes()).padStart(2, "0");
    const seconds = String(now.getSeconds()).padStart(2, "0");
    const ampm = hours >= 12 ? "PM" : "AM";

    hours = hours % 12;
    hours = hours ? hours : 12; // 0 should be 12
    const hoursStr = String(hours).padStart(2, "0");

    timeString = `${hoursStr}:${minutes}:${seconds}`;
    ampmString = ampm;

    // Format Day, Month Date, Year
    const options: Intl.DateTimeFormatOptions = {
      weekday: "short",
      month: "short",
      day: "numeric",
      year: "numeric",
    };
    dateString = now.toLocaleDateString(undefined, options);
  }

  onMount(() => {
    updateClock();
    const interval = setInterval(updateClock, 1000);
    return () => clearInterval(interval);
  });
</script>

<div class="digital-clock-container">
  <div class="clock-card">
    <div class="time-wrapper">
      <span class="time">{timeString}</span>
      <span class="ampm">{ampmString}</span>
    </div>
    <div class="date-badge">
      <span class="date">{dateString}</span>
    </div>
  </div>
</div>

<style>
  .digital-clock-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    padding: 0.5rem 1rem 0.25rem 1rem;
    z-index: 10;
    user-select: none;
  }

  .clock-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(8px);
    border-radius: 12px;
    padding: 0.4rem 1.4rem;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
  }

  .time-wrapper {
    display: flex;
    align-items: baseline;
    gap: 0.35rem;
  }

  .time {
    font-family: monospace, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto;
    font-size: 1.6rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: #ffffff;
    text-shadow: 0 0 12px rgba(255, 255, 255, 0.25);
    line-height: 1.1;
  }

  .ampm {
    font-size: 0.75rem;
    font-weight: 600;
    color: #60a5fa;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .date-badge {
    margin-top: 0.15rem;
  }

  .date {
    font-size: 0.8rem;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.65);
    letter-spacing: 0.04em;
  }
</style>
