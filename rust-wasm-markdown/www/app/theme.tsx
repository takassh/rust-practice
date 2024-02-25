import { extendTheme } from "@chakra-ui/react";
import { mode } from '@chakra-ui/theme-tools'

export const theme = extendTheme({
    shadows: {
        outline: 'none',
    },
    fonts: {
        heading:
            'Inter, Helvetica Neue, Helvetica, Hiragino Sans, Hiragino Kaku Gothic ProN, Arial, Yu Gothic, Meiryo, sans-serif',
        body: 'Inter, Noto Sans JP, Hiragino Kaku Gothic ProN, Proxima Nova, Verdana, 游ゴシック, YuGothic, Meiryo, sans-serif',
    },
    useSystemColorMode: true,
    styles: {
        global: (props: any) => ({
            body: {
                bg: mode('gray.50', 'gray.800')(props),
            },
        }),
    },
})