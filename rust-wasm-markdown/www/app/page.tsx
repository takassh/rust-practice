'use client'

import { Flex, Text, Box, Textarea } from "@chakra-ui/react";
import { ChangeEventHandler, useState } from "react";
import { to_html } from 'rust-wasm-markdown';

type ParseHtmlFunction = (input: string) => string;

async function getParser() {
  const wasm = await import("rust-wasm-markdown");
  let parser: { to_html: typeof to_html };
  parser = wasm;
  return parser;
}

export default function Home() {
  const [html, setHtml] = useState('')
  const [value, setValue] = useState('')
  const handleChange: ChangeEventHandler<HTMLTextAreaElement> = async (event) => {
    setValue(event.target.value);
    setHtml((await getParser()).to_html(event.target.value))
  }

  console.log(html);
  return (<Flex color='white'>
    <Box flex={1} bg='tomato'>
      <Textarea placeholder='Input Markdown' value={value} onChange={handleChange} />
    </Box>
    <Box w='4' h='10' bg='teal.500' />
    <Box flex={1} bg='tomato' pl={4}>
      <Text dangerouslySetInnerHTML={{ __html: html }} />
    </Box>
  </Flex>);
}