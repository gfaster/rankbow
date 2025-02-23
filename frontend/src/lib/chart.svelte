<script>
    import * as d3 from "d3";

    // set the dimensions and margins of the graph
    const margin = { top: 10, right: 30, bottom: 20, left: 50 },
        width = 460 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;

    // append the svg object to the body of the page
    const svg = d3
        .select("#my_dataviz")
        .append("svg")
        .attr("width", width + margin.left + margin.right)
        .attr("height", height + margin.top + margin.bottom)
        .append("g")
        .attr("transform", `translate(${margin.left},${margin.top})`);

    // Parse the Data
    d3.csv(
        "https://raw.githubusercontent.com/holtzy/D3-graph-gallery/master/DATA/data_stackedXL.csv",
    ).then(function (data) {
        // List of subgroups = header of the csv files = soil condition here
        const subgroups = data.columns.slice(1);

        // List of groups = species here = value of the first column called group -> I show them on the X axis
        const groups = data.map((d) => d.group);

        // Add X axis
        const x = d3
            .scaleBand()
            .domain(groups)
            .range([0, width])
            .padding([0.2]);
        svg.append("g")
            .attr("transform", `translate(0, ${height})`)
            .call(d3.axisBottom(x).tickSizeOuter(0));

        // Add Y axis
        const y = d3.scaleLinear().domain([0, 120]).range([height, 0]);
        svg.append("g").call(d3.axisLeft(y));

        // color palette = one color per subgroup
        const color = d3.scaleOrdinal().domain(subgroups).range(d3.schemeSet2);

        //stack the data? --> stack per subgroup
        const stackedData = d3.stack().keys(subgroups)(data);

        // ----------------
        // Highlight a specific subgroup when hovered
        // ----------------

        // Show the bars
        svg.append("g")
            .selectAll("g")
            // Enter in the stack data = loop key per key = group per group
            .data(stackedData)
            .join("g")
            .attr("fill", (d) => color(d.key))
            .attr("class", (d) => "myRect " + d.key) // Add a class to each subgroup: their name
            .selectAll("rect")
            // enter a second time = loop subgroup per subgroup to add all rectangles
            .data((d) => d)
            .join("rect")
            .attr("x", (d) => x(d.data.group))
            .attr("y", (d) => y(d[1]))
            .attr("height", (d) => y(d[0]) - y(d[1]))
            .attr("width", x.bandwidth())
            .attr("stroke", "grey")
            .on("mouseover", function (event, d) {
                // What happens when user hover a bar

                // what subgroup are we hovering?
                const subGroupName = d3.select(this.parentNode).datum().key;

                // Reduce opacity of all rect to 0.2
                d3.selectAll(".myRect").style("opacity", 0.2);

                // Highlight all rects of this subgroup with opacity 1. It is possible to select them since they have a specific class = their name.
                d3.selectAll("." + subGroupName).style("opacity", 1);
            })
            .on("mouseleave", function (event, d) {
                // When user do not hover anymore

                // Back to normal opacity: 1
                d3.selectAll(".myRect").style("opacity", 1);
            });
    });
</script>

<svg id="barchart"></svg>
