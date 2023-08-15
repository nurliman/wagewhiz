const svgToDataUri = require("mini-svg-data-uri");
const plugin = require("tailwindcss/plugin");
const defaultTheme = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");
const [baseFontSize, { lineHeight: baseLineHeight }] = defaultTheme.fontSize.base;
const { spacing, borderWidth, borderRadius } = defaultTheme;

module.exports = () => {
  return plugin(
    ({ addBase, theme }) => {
      addBase({
        [[
          "[type='text']",
          "[type='email']",
          "[type='url']",
          "[type='password']",
          "[type='number']",
          "[type='date']",
          "[type='datetime-local']",
          "[type='month']",
          "[type='search']",
          "[type='tel']",
          "[type='time']",
          "[type='week']",
          "[multiple]",
          "textarea",
          "select",
        ]]: {
          appearance: "none",
          "background-color": "#fff",
          "border-color": theme("colors.gray.500", colors.gray[500]),
          "border-width": borderWidth["DEFAULT"],
          "border-radius": borderRadius.none,
          "padding-top": spacing[2],
          "padding-right": spacing[3],
          "padding-bottom": spacing[2],
          "padding-left": spacing[3],
          "font-size": baseFontSize,
          "line-height": baseLineHeight,
          "--tw-shadow": "0 0 #0000",
          "&:focus": {
            outline: "2px solid transparent",
            "outline-offset": "2px",
            "--tw-ring-inset": "var(--tw-empty,/*!*/ /*!*/)",
            "--tw-ring-offset-width": "0px",
            "--tw-ring-offset-color": "#fff",
            "--tw-ring-color": theme("colors.blue.600", colors.blue[600]),
            "--tw-ring-offset-shadow": `var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)`,
            "--tw-ring-shadow": `var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color)`,
            "box-shadow": `var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow)`,
            "border-color": theme("colors.blue.600", colors.blue[600]),
          },
        },
        [["input::placeholder", "textarea::placeholder"]]: {
          color: theme("colors.gray.500", colors.gray[500]),
          opacity: "1",
        },
        ["::-webkit-datetime-edit-fields-wrapper"]: {
          padding: "0",
        },
        ["::-webkit-date-and-time-value"]: {
          "min-height": "1.5em",
        },
        ["select:not([size])"]: {
          "background-image": `url("${svgToDataUri(
            `<svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                    <path stroke="${theme(
                      "colors.gray.500",
                      colors.gray[500],
                    )}" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                  </svg>`,
          )}")`,
          "background-position": `right ${spacing[3]} center`,
          "background-repeat": `no-repeat`,
          "background-size": `0.75em 0.75em`,
          "padding-right": spacing[10],
          "print-color-adjust": `exact`,
        },
        ["[multiple]"]: {
          "background-image": "initial",
          "background-position": "initial",
          "background-repeat": "unset",
          "background-size": "initial",
          "padding-right": spacing[3],
          "print-color-adjust": "unset",
        },
        [[`[type='checkbox']`, `[type='radio']`]]: {
          appearance: "none",
          padding: "0",
          "print-color-adjust": "exact",
          display: "inline-block",
          "vertical-align": "middle",
          "background-origin": "border-box",
          "user-select": "none",
          "flex-shrink": "0",
          height: spacing[4],
          width: spacing[4],
          color: theme("colors.blue.600", colors.blue[600]),
          "background-color": "#fff",
          "border-color": theme("colors.gray.500", colors.gray[500]),
          "border-width": borderWidth["DEFAULT"],
          "--tw-shadow": "0 0 #0000",
        },
        [`[type='checkbox']`]: {
          "border-radius": borderRadius["none"],
        },
        [`[type='radio']`]: {
          "border-radius": "100%",
        },
        [[`[type='checkbox']:focus`, `[type='radio']:focus`]]: {
          outline: "2px solid transparent",
          "outline-offset": "2px",
          "--tw-ring-inset": "var(--tw-empty,/*!*/ /*!*/)",
          "--tw-ring-offset-width": "2px",
          "--tw-ring-offset-color": "#fff",
          "--tw-ring-color": theme("colors.blue.600", colors.blue[600]),
          "--tw-ring-offset-shadow": `var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)`,
          "--tw-ring-shadow": `var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color)`,
          "box-shadow": `var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow)`,
        },
        [[
          `[type='checkbox']:checked`,
          `[type='radio']:checked`,
          `.dark [type='checkbox']:checked`,
          `.dark [type='radio']:checked`,
        ]]: {
          "border-color": `transparent`,
          "background-color": `currentColor`,
          "background-size": `0.55em 0.55em`,
          "background-position": `center`,
          "background-repeat": `no-repeat`,
        },
        [`[type='checkbox']:checked`]: {
          "background-image": `url("${svgToDataUri(
            `<svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 16 12">
                        <path stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M1 5.917 5.724 10.5 15 1.5"/>
                    </svg>`,
          )}")`,
          "background-repeat": `no-repeat`,
          "background-size": `0.55em 0.55em`,
          "print-color-adjust": `exact`,
        },
        [`[type='radio']:checked`]: {
          "background-image": `url("${svgToDataUri(
            `<svg viewBox="0 0 16 16" fill="white" xmlns="http://www.w3.org/2000/svg"><circle cx="8" cy="8" r="3"/></svg>`,
          )}")`,
          "background-size": `1em 1em`,
        },
        [`.dark [type='radio']:checked`]: {
          "background-image": `url("${svgToDataUri(
            `<svg viewBox="0 0 16 16" fill="white" xmlns="http://www.w3.org/2000/svg"><circle cx="8" cy="8" r="3"/></svg>`,
          )}")`,
          "background-size": `1em 1em`,
        },
        [`[type='checkbox']:indeterminate`]: {
          "background-image": `url("${svgToDataUri(
            `<svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 16 12">
                    <path stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M1 5.917 5.724 10.5 15 1.5"/>
                    </svg>`,
          )}")`,
          "background-color": `currentColor`,
          "border-color": `transparent`,
          "background-position": `center`,
          "background-repeat": `no-repeat`,
          "background-size": `0.55em 0.55em`,
          "print-color-adjust": `exact`,
        },
        [[`[type='checkbox']:indeterminate:hover`, `[type='checkbox']:indeterminate:focus`]]: {
          "border-color": "transparent",
          "background-color": "currentColor",
        },
        [`[type='file']`]: {
          background: "unset",
          "border-color": "inherit",
          "border-width": "0",
          "border-radius": "0",
          padding: "0",
          "font-size": "unset",
          "line-height": "inherit",
        },
        [`[type='file']:focus`]: {
          outline: `1px auto inherit`,
        },
        [[`input[type=file]::file-selector-button`]]: {
          color: "white",
          background: theme("colors.gray.800", colors.gray[800]),
          border: 0,
          "font-weight": theme("fontWeight.medium"),
          "font-size": theme("fontSize.sm"),
          cursor: "pointer",
          "padding-top": spacing[2.5],
          "padding-bottom": spacing[2.5],
          "padding-left": spacing[8],
          "padding-right": spacing[4],
          "margin-inline-start": "-1rem",
          "margin-inline-end": "1rem",
          "&:hover": {
            background: theme("colors.gray.700", colors.gray[700]),
          },
        },
        [[`.dark input[type=file]::file-selector-button`]]: {
          color: "white",
          background: theme("colors.gray.600", colors.gray[600]),
          "&:hover": {
            background: theme("colors.gray.500", colors.gray[500]),
          },
        },
        [[`input[type="range"]::-webkit-slider-thumb`]]: {
          height: spacing[5],
          width: spacing[5],
          background: theme("colors.blue.600", colors.blue[600]),
          "border-radius": borderRadius.full,
          border: 0,
          appearance: "none",
          "-moz-appearance": "none",
          "-webkit-appearance": "none",
          cursor: "pointer",
        },
        [[`input[type="range"]:disabled::-webkit-slider-thumb`]]: {
          background: theme("colors.gray.400", colors.gray[400]),
        },
        [[`.dark input[type="range"]:disabled::-webkit-slider-thumb`]]: {
          background: theme("colors.gray.500", colors.gray[500]),
        },
        [[`input[type="range"]:focus::-webkit-slider-thumb`]]: {
          outline: "2px solid transparent",
          "outline-offset": "2px",
          "--tw-ring-offset-shadow":
            "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
          "--tw-ring-shadow":
            "var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color)",
          "box-shadow":
            "var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)",
          "--tw-ring-opacity": 1,
          "--tw-ring-color": "rgb(164 202 254 / var(--tw-ring-opacity))",
        },
        [[`input[type="range"]::-moz-range-thumb`]]: {
          height: spacing[5],
          width: spacing[5],
          background: theme("colors.blue.600", colors.blue[600]),
          "border-radius": borderRadius.full,
          border: 0,
          appearance: "none",
          "-moz-appearance": "none",
          "-webkit-appearance": "none",
          cursor: "pointer",
        },
        [[`input[type="range"]:disabled::-moz-range-thumb`]]: {
          background: theme("colors.gray.400", colors.gray[400]),
        },
        [[`.dark input[type="range"]:disabled::-moz-range-thumb`]]: {
          background: theme("colors.gray.500", colors.gray[500]),
        },
        [[`input[type="range"]::-moz-range-progress`]]: {
          background: theme("colors.blue.500", colors.blue[500]),
        },
        [[`input[type="range"]::-ms-fill-lower`]]: {
          background: theme("colors.blue.500", colors.blue[500]),
        },
        [[`input[type="range"].range-sm::-webkit-slider-thumb`]]: {
          height: spacing[4],
          width: spacing[4],
        },
        [[`input[type="range"].range-lg::-webkit-slider-thumb`]]: {
          height: spacing[6],
          width: spacing[6],
        },
        [[`input[type="range"].range-sm::-moz-range-thumb`]]: {
          height: spacing[4],
          width: spacing[4],
        },
        [[`input[type="range"].range-lg::-moz-range-thumb`]]: {
          height: spacing[6],
          width: spacing[6],
        },
      });
    },
    {
      darkMode: "class", // or 'media' or 'class',
      theme: {
        extend: {
          height: {
            modal: "calc(100% - 2rem)",
          },
          boxShadow: {
            "sm-light": "0 2px 5px 0px rgba(255, 255, 255, 0.08)",
          },
        },
      },
    },
  );
};
