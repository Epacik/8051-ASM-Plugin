import { kits } from '../../constants/kits';
import * as log from 'loglevel'
import { getGlobalSettings } from '../../server'
import { dsm51 } from './dsm51/labels';

export const getKitLabels = () : Array<string> => {
	let kit = getGlobalSettings().kit;
	log.info(`selected kit: ${kit}\n settings:`);
	log.info(getGlobalSettings());

	if(kit == kits.dsm51) return dsm51.labels;

	return [];
}