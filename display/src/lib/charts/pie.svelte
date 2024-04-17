<script lang="ts">
  // import Chart from 'chart.js/auto'
  import Chart from 'chart.js/auto';
  import { onMount } from 'svelte';
  export let unsafe;
  export let total;
  export let name;

  console.log(unsafe, total, name);

	let ctx;
	let chartCanvas;


  const data = {
    labels: ['Unsafe', 'Safe'],
    datasets: [
      {
        label: name,
        data: [unsafe, total - unsafe],
        backgroundColor: [
          'rgb(255, 99, 132)',
          'rgb(54, 162, 235)',
        ],
        // borderColor: 'rgb(255, 99, 132)',
      },
    ],
  };

  const options = {
    responsive: true,
    plugins: {
      legend: {
        position: 'top',
      },
      title: {
        display: true,
        text: name,
      }
    }
  };


	onMount(() => {
		  ctx = chartCanvas.getContext('2d');
			var chart = new Chart(ctx, {
				type: 'pie',
				data: data,
        // options: options,
		});
	});


  export { PieChart }
</script>

<canvas bind:this={chartCanvas}></canvas>
