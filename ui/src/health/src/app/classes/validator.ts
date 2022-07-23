import { AbstractControl, ValidationErrors, ValidatorFn } from "@angular/forms";

export class Validator {

    static pattern(
        pattern: RegExp,
        error: ValidationErrors
    ): ValidatorFn {
        return (control: AbstractControl): ValidationErrors | null => {
            if (!control.value) {
                return null;
            }

            const valid = pattern.test(control.value);
            return valid ? null : error;
        }
    }
}
