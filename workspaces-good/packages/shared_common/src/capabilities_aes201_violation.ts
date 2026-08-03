// AES201: capabilities importing from surface (forbidden layer)
import { SurfaceBadCommand } from '../cli_commands/surface_bad_command';

export class ForbiddenImportChecker {
    check(): boolean {
        const cmd = new SurfaceBadCommand();
        return true;
    }
}
