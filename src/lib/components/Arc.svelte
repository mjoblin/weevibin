<script lang="ts">
    export let radius: number = 50;
    export let thickness: number = 5;
    export let progress: number = 360;
    export let color: string = "orange";
    export let trackColor: string = "black";

    // Approach to drawing the arc:
    // http://www.independent-software.com/drawing-progress-arc-in-pure-css-using-skewed-rectangles.html

    const MAX_PROGRESS = 359.999;

    $: arcContainerSize = radius * 2;
    $: arcCenterSVG = radius;
    $: radiusSVG = radius - thickness;
    $: progressDisplay = progress < 0 ? 0 : progress >= 360 ? MAX_PROGRESS : progress;

    // SVG: origin (0, 0) is the top left; +x goes right, +y goes down
    // https://stackoverflow.com/questions/5736398/how-to-calculate-the-svg-path-for-an-arc-of-a-circle

    const polarToCartesian = (centerX: number, centerY: number, radius: number, angleInDegrees: number) => {
        var angleInRadians = (angleInDegrees-90) * Math.PI / 180.0;

        return {
            x: centerX + (radius * Math.cos(angleInRadians)),
            y: centerY + (radius * Math.sin(angleInRadians))
        };
    };

    const describeArc = (x: number, y: number, radius: number, startAngle: number, endAngle: number) => {
        var start = polarToCartesian(x, y, radius, endAngle);
        var end = polarToCartesian(x, y, radius, startAngle)

        var largeArcFlag = endAngle - startAngle <= 180 ? "0" : "1";

        var d = [
            "M", start.x, start.y,
            "A", radius, radius, 0, largeArcFlag, 0, end.x, end.y
        ].join(" ");

        return d;
    };

    $: cssVarStyles =
        `--arc-container-size:${arcContainerSize}px;` +
        `--thickness:${thickness};` +
        `--progress-color:${color};` +
        `--track-color:${trackColor}`;
</script>

<div class="Arc" style={cssVarStyles}>
    <svg class="arc-svg" xmlns="http://www.w3.org/2000/svg">
        <path class="arc-track" d={describeArc(arcCenterSVG, arcCenterSVG, radiusSVG, 0, MAX_PROGRESS)} />
        <path class="arc-progress" d={describeArc(arcCenterSVG, arcCenterSVG, radiusSVG, 0, progressDisplay)} />
    </svg>

    <div class="arc-content">
        <slot />
    </div>
</div>

<style>
    .Arc {
        width: var(--arc-container-size);
        height: var(--arc-container-size);
        position: relative;

        & .arc-svg {
            width: 100%;
            height: 100%;
            stroke-width: var(--thickness);
            stroke-linecap: round;
        }
    }

    .arc-track {
        stroke: var(--track-color);
        fill: none;
    }

    .arc-progress {
        stroke: var(--progress-color);
        fill: none;
    }

    .arc-content {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>