import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const myCustomTheme: CustomThemeConfig = {
  name: "my-custom-theme",
  properties: {
    // =~= Theme Properties =~=
    "--theme-font-family-base":
      '"Nunito Variable", "Nunito", "Helvetica Neue", Helvetica, Arial,sans-serif',
    "--theme-font-family-heading":
      '"Nunito Variable", "Nunito", "Helvetica Neue", Helvetica, Arial,sans-serif',
    "--theme-font-color-base": "0 0 0",
    "--theme-font-color-dark": "255 255 255",
    "--theme-rounded-base": "8px",
    "--theme-rounded-container": "8px",
    "--theme-border-base": "1px",
    // =~= Theme On-X Colors =~=
    "--on-primary": "255 255 255",
    "--on-secondary": "0 0 0",
    "--on-tertiary": "0 0 0",
    "--on-success": "0 0 0",
    "--on-warning": "0 0 0",
    "--on-error": "255 255 255",
    "--on-surface": "255 255 255",
    // =~= Theme Colors  =~=
    // primary | #264653
    "--color-primary-50": "222 227 229", // #dee3e5
    "--color-primary-100": "212 218 221", // #d4dadd
    "--color-primary-200": "201 209 212", // #c9d1d4
    "--color-primary-300": "168 181 186", // #a8b5ba
    "--color-primary-400": "103 126 135", // #677e87
    "--color-primary-500": "38 70 83", // #264653
    "--color-primary-600": "34 63 75", // #223f4b
    "--color-primary-700": "29 53 62", // #1d353e
    "--color-primary-800": "23 42 50", // #172a32
    "--color-primary-900": "19 34 41", // #132229
    // secondary | #2A9D8F
    "--color-secondary-50": "223 240 238", // #dff0ee
    "--color-secondary-100": "212 235 233", // #d4ebe9
    "--color-secondary-200": "202 231 227", // #cae7e3
    "--color-secondary-300": "170 216 210", // #aad8d2
    "--color-secondary-400": "106 186 177", // #6abab1
    "--color-secondary-500": "42 157 143", // #2A9D8F
    "--color-secondary-600": "38 141 129", // #268d81
    "--color-secondary-700": "32 118 107", // #20766b
    "--color-secondary-800": "25 94 86", // #195e56
    "--color-secondary-900": "21 77 70", // #154d46
    // tertiary | #E9C46A
    "--color-tertiary-50": "252 246 233", // #fcf6e9
    "--color-tertiary-100": "251 243 225", // #fbf3e1
    "--color-tertiary-200": "250 240 218", // #faf0da
    "--color-tertiary-300": "246 231 195", // #f6e7c3
    "--color-tertiary-400": "240 214 151", // #f0d697
    "--color-tertiary-500": "233 196 106", // #E9C46A
    "--color-tertiary-600": "210 176 95", // #d2b05f
    "--color-tertiary-700": "175 147 80", // #af9350
    "--color-tertiary-800": "140 118 64", // #8c7640
    "--color-tertiary-900": "114 96 52", // #726034
    // success | #3A86FF
    "--color-success-50": "225 237 255", // #e1edff
    "--color-success-100": "216 231 255", // #d8e7ff
    "--color-success-200": "206 225 255", // #cee1ff
    "--color-success-300": "176 207 255", // #b0cfff
    "--color-success-400": "117 170 255", // #75aaff
    "--color-success-500": "58 134 255", // #3A86FF
    "--color-success-600": "52 121 230", // #3479e6
    "--color-success-700": "44 101 191", // #2c65bf
    "--color-success-800": "35 80 153", // #235099
    "--color-success-900": "28 66 125", // #1c427d
    // warning | #F77F00
    "--color-warning-50": "254 236 217", // #feecd9
    "--color-warning-100": "253 229 204", // #fde5cc
    "--color-warning-200": "253 223 191", // #fddfbf
    "--color-warning-300": "252 204 153", // #fccc99
    "--color-warning-400": "249 165 77", // #f9a54d
    "--color-warning-500": "247 127 0", // #F77F00
    "--color-warning-600": "222 114 0", // #de7200
    "--color-warning-700": "185 95 0", // #b95f00
    "--color-warning-800": "148 76 0", // #944c00
    "--color-warning-900": "121 62 0", // #793e00
    // error | #D62828
    "--color-error-50": "249 223 223", // #f9dfdf
    "--color-error-100": "247 212 212", // #f7d4d4
    "--color-error-200": "245 201 201", // #f5c9c9
    "--color-error-300": "239 169 169", // #efa9a9
    "--color-error-400": "226 105 105", // #e26969
    "--color-error-500": "214 40 40", // #D62828
    "--color-error-600": "193 36 36", // #c12424
    "--color-error-700": "161 30 30", // #a11e1e
    "--color-error-800": "128 24 24", // #801818
    "--color-error-900": "105 20 20", // #691414
    // surface | #2A3342
    "--color-surface-50": "223 224 227", // #dfe0e3
    "--color-surface-100": "212 214 217", // #d4d6d9
    "--color-surface-200": "202 204 208", // #caccd0
    "--color-surface-300": "170 173 179", // #aaadb3
    "--color-surface-400": "106 112 123", // #6a707b
    "--color-surface-500": "42 51 66", // #2A3342
    "--color-surface-600": "38 46 59", // #262e3b
    "--color-surface-700": "32 38 50", // #202632
    "--color-surface-800": "25 31 40", // #191f28
    "--color-surface-900": "21 25 32", // #151920
  },
};
