<script lang="ts">
    import { stringToColour } from "../../mock";

    export let name: string;
    export let color: string | undefined = undefined;
</script>

<span
    style={`background-color: ${color ?? stringToColour(name)}`}
    class="badge text-warning-50 mr-1">{name}</span
>
