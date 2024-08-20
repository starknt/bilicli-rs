#!/bin/env node

import { cac } from 'cac'
import { version } from '../package.json'
import { App, AppOptions, EditOptions } from './app'

const cli = cac('bilicli')

cli
  .command('<room_id>', 'Input will be listen room id, and open live console')
  .option('-c, --cookie <cookie>', 'Cookie for login')
  .option('--config <config>', 'Config file path')
  .action(async (roomId: string, options: AppOptions) => {
    const app = new App(~~roomId, options)
    await app.run()
    process.exit(0)
  })

cli
  .command('edit', 'Open editor to edit config')
  .option('-c, --config <config>', 'Custom config file path', { default: 'biliclirc' })
  .action((options: EditOptions) => {
    // TODO: open editor to edit config
    console.log(options)
  })

cli.help()
cli.version(version)

cli.parse()

