import * as d3 from "https://d3js.org/d3.v6.min.js";

export async function piChart(width, height, dataset) {
  try {
    let w = width;
    let h = height;
    let outerRadius = Math.min(w, h) / 2;
    let innerRadius = outerRadius * 0.3;
    let color = d3.scaleOrdinal(d3.schemeCategory10);

    let vis = d3
      .select("#pie")
      .append("svg")
      .attr("width", width)
      .attr("height", height)
      .append("g")
      .attr("transform", `translate(${outerRadius},${outerRadius})`);

    let arc = d3.arc().innerRadius(innerRadius).outerRadius(outerRadius);

    let pie = d3.pie().value(function (d) {
      return d.value;
    })(dataset);

    vis
      .selectAll("path")
      .data(pie)
      .enter()
      .append("path")
      .attr("d", arc)
      .attr("fill", (d, i) => color(i));
  } catch (err) {
    throw err;
  }
}
