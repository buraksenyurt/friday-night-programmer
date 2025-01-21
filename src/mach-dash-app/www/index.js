import init, { analyze_stats } from '../pkg/mach_dash_app.js';

await init();

const trendChart = new Chart(document.getElementById('trendChart'), {
    type: 'line',
    data: {
        labels: [],
        datasets: [
            {
                label: 'CPU Usage (%)',
                data: [],
                borderColor: 'rgba(75, 192, 192, 1)',
                tension: 0.1,
            },
            {
                label: 'Memory Used (MB)',
                data: [],
                borderColor: 'rgba(153, 102, 255, 1)',
                tension: 0.1,
            },
        ],
    },
});

async function updateStats() {
    const response = await fetch('http://localhost:6501/machine/stats');
    const data = await response.json();

    const analysis = await analyze_stats(JSON.stringify(data));
    const result = JSON.parse(analysis);

    document.getElementById('avgCpu').textContent = result.avg_cpu_usage.toFixed(2);
    document.getElementById('avgMemory').textContent = (result.avg_memory_used / 1024).toFixed(2);

    trendChart.data.labels = data.map((d) =>
        new Date(d.timestamp * 1000).toLocaleTimeString()
    );
    trendChart.data.datasets[0].data = data.map((d) => d.cpu_usage);
    trendChart.data.datasets[1].data = data.map((d) => d.memory_used / 1024);
    trendChart.update();
}

setInterval(updateStats, 5000);
updateStats();
