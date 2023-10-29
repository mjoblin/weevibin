/**
 * Get the color value for the given cssVarName.
 *
 * Expects the CSS var's value to be in #rrggbb[aa] format.
 */
export const colorFromCssVar = (cssVarName: string): string | undefined => {
    const cssValue = getComputedStyle(document.body).getPropertyValue(cssVarName);

    if (cssValue.match(/^#[a-fA-F0-9]{6,8}$/)) {
        return cssValue;
    }

    return undefined;
}