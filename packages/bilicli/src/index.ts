#!/bin/env node

import { cac } from 'cac'
import { Cli } from '@natmri/bilicli-napi'
import { version } from '../package.json'

const cli = cac('bilicli')

cli.command('<room_id>', 'Open live console')
  .action(async (roomId: string) => {
    const cli = new Cli(~~roomId)

    cli.run()
  })

cli.help()
cli.version(version)

cli.parse()

