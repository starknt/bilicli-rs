#!/bin/env node

import { cac } from 'cac'
import { version } from '../package.json'
import { App } from './app'

const cli = cac('bilicli')

cli.command('<room_id>', 'Open live console')
  .action(async (roomId: string) => {
    const app = new App(~~roomId)

    await app.run()
    process.exit(0)
  })

cli.help()
cli.version(version)

cli.parse()

