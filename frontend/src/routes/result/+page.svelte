<script>
    import { scaleBand, scaleLinear } from "d3-scale";
    import { stack, stackOrderNone } from "d3-shape"; // Functions to stack data for the chart
    import { max } from "d3-array";
    import AxisLeft from "./AxisLeftV5.svelte";
    import XAxisLabel from "./XAxisLabel.svelte";
    import YAxisLabel from "./YAxisLabel.svelte";

    // API fetch
    import { getData } from "./utils.js";

    // Props with default values
    // let {
    //     title = "Ballot Results",
    //     // labelCategories must be overridden by the time we get back data from Gav
    //     labelCategories = ["A", "B", "C", "D", "E"],
    //     singleStack = null,
    // } = $props();

    // Dataset must be JSON for ballot by Gavin
    // create JS function to await promise and return data
    let content = getData();

    let { singleStack = null } = $props();
    let labelCategories = content.choices;
    let title = content.title;

    console.log(content);
    let ranks = content.rank_fields;
    let data = content.votes[content.votes.length - 1];

    // let data = [
    //     {
    //         title: "Winter",
    //         A: 15,
    //         B: 5,
    //         C: 2,
    //         D: 20,
    //         E: 6,
    //     },
    //     {
    //         title: "Spring",
    //         A: 25,
    //         B: 18,
    //         C: 10,
    //         D: 28,
    //         E: 12,
    //     },
    //     {
    //         title: "Summer",
    //         A: 60,
    //         B: 50,
    //         C: 40,
    //         D: 70,
    //         E: 35,
    //     },
    //     {
    //         title: "Fall",
    //         A: 35,
    //         B: 28,
    //         C: 15,
    //         D: 40,
    //         E: 20,
    //     },
    // ];

    // Margins around the chart to position it properly inside the SVG container
    const margin = { top: 25, right: 30, bottom: 100, left: 32 };

    // Dimensions of the chart
    let width = $state(480); // Chart width (reactive using Svelte's `$state`)
    const height = 370; // Chart height (constant)

    // Colors for each category in the stacked bar chart
    const colors = ["#ffbe0b", "#FB5607", "#FF006E", "#8338EC", "#3A86FF"];

    // X Scale: Maps the '"title"' categories to horizontal positions
    let xScale = $derived(
        scaleBand()
            .domain(data.map((d) => d["title"])) // Categories (Winter, Spring, etc.)
            .range([margin.left, width - margin.right])
            .padding(0.2), // Adds padding between bars
    );

    // Y Scale: Maps the stacked sum of categories to vertical positions
    const yScale = scaleLinear()
        .domain([
            0,
            max(
                data,
                (d) =>
                    ranks.reduce(
                        (sum, key) => sum + (d[key] !== undefined ? d[key] : 0),
                        0,
                    ), // Calculates the total for each season
            ),
        ])
        .nice() // Adjusts the domain to end at a "nice" round number
        .range([height - margin.bottom, margin.top]); // Pixel range for the y-axis (inverted as SVG origin is top-left)

    // Stack generator: Prepares the data for stacking
    const stackGenerator = stack()
        .keys(ranks) // Keys (A, B, etc.) to stack on top of each other
        .order(stackOrderNone); // No specific order for stacking

    // Generates the stacked data structure for the chart
    const stackedData = stackGenerator(data); // Array of layers for each category
    let alternativeData = $state(null);

    if (singleStack !== null) {
        alternativeData = stackedData[singleStack];
    }
</script>

<div
    class=" p relative box-border min-w-full rounded-xl border-gray-100 p-4 pt-0"
    bind:clientWidth={width}
>
    <div
        class="flex w-full items-center justify-between pb-4 pt-1 font-semibold text-gray-600"
    >
        <h3 class="">{"title"}</h3>
    </div>
    <svg width={width - margin.left - margin.right} {height}>
        <!-- Y-axis -->
        <g>
            {#each yScale.ticks(5) as tick}
                <text
                    x={margin.left - 10}
                    y={yScale(tick)}
                    font-size="20px"
                    text-anchor="end"
                    alignment-baseline="middle"
                >
                    <!-- {tick} -->
                </text>
                <line
                    class="stroke-gray-300"
                    stroke-dasharray="6,6"
                    x1={margin.left + 10}
                    x2={width - margin.right - margin.left}
                    y1={yScale(tick)}
                    y2={yScale(tick)}
                />
            {/each}
        </g>

        <AxisLeft {width} {height} {margin} {yScale} ticksNumber={5} />

        <!-- X and Y Axis Labels -->
        <XAxisLabel {width} {height} {margin} label={"Seasons"} />
        <YAxisLabel
            {width}
            {height}
            {margin}
            xoffset={0}
            textanchor={"start"}
            position={"top"}
            label={"Total Sales  ↑"}
        />

        <!-- Bars and Total Values -->

        <!-- Render all stacks -->
        {#each stackedData as series, i}
            {#each series as [y0, y1], j}
                <rect
                    rx="3"
                    ry="3"
                    x={xScale(data[j]["title"])}
                    y={yScale(y1)}
                    width={xScale.bandwidth()}
                    height={yScale(y0) - yScale(y1)}
                    opacity={singleStack !== null && i !== singleStack
                        ? 0.1
                        : 1}
                    fill={colors[i]}
                />
            {/each}
        {/each}

        <!-- X-axis labels -->
        <g transform={`translate(0, ${height - margin.bottom})`}>
            {#each xScale.domain() as period}
                <text
                    class="fill-gray-300"
                    x={xScale(period) + xScale.bandwidth() / 2}
                    y="20"
                    font-size="14px"
                    text-anchor="middle"
                >
                    {period}
                </text>
            {/each}
        </g>

        <!-- Category Labels with Color Indicators -->
        <g transform={`translate(14, ${height - 20})`}>
            {#each ranks as category, i}
                <g
                    transform={`translate(${margin.left + (i * width) / 7 + xScale.bandwidth() - 70}, -4)`}
                >
                    <!-- Color box -->
                    <rect
                        style="border-radius:10px;"
                        width="16"
                        height="16"
                        rx="4"
                        ry="4"
                        fill={colors[i]}
                    />
                    <!-- Category text -->
                    <text
                        class="fill-gray-300"
                        x="20"
                        y="10"
                        font-size="14px"
                        alignment-baseline="middle">{category}</text
                    >
                </g>
            {/each}
        </g>
    </svg>
</div>
