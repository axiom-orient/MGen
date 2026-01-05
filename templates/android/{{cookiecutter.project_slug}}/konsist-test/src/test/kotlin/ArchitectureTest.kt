package {{ cookiecutter.base_package }}.konsist

import com.lemonappdev.konsist.api.Konsist
import com.lemonappdev.konsist.api.ext.list.withNameEndingWith
import com.lemonappdev.konsist.api.verify.assertTrue
import org.junit.jupiter.api.Test

class ArchitectureTest {

    @Test
    fun `all ViewModels must end with ViewModel suffix`() {
        Konsist
            .scopeFromProject()
            .classes()
            .withNameEndingWith("ViewModel")
            .assertTrue { it.name.endsWith("ViewModel") }
    }

    @Test
    fun `all ViewModels must reside in feature package`() {
        Konsist
            .scopeFromProject()
            .classes()
            .withNameEndingWith("ViewModel")
            .assertTrue { it.resideInPackage("..feature..") }
    }

    @Test
    fun `all UseCase interfaces must end with UseCase suffix`() {
        Konsist
            .scopeFromProject()
            .interfaces()
            .withNameEndingWith("UseCase")
            .assertTrue { it.name.endsWith("UseCase") }
    }

    @Test
    fun `all UseCase interfaces must reside in domain package`() {
        Konsist
            .scopeFromProject()
            .interfaces()
            .withNameEndingWith("UseCase")
            .assertTrue { it.resideInPackage("..domain..") }
    }

    @Test
    fun `all Service interfaces must end with Service suffix`() {
        Konsist
            .scopeFromProject()
            .interfaces()
            .withNameEndingWith("Service")
            .assertTrue { it.name.endsWith("Service") }
    }

    @Test
    fun `all Service interfaces must reside in service package`() {
        Konsist
            .scopeFromProject()
            .interfaces()
            .withNameEndingWith("Service")
            .assertTrue { it.resideInPackage("..service..") }
    }

    @Test
    fun `feature layer should not depend on other features`() {
        Konsist
            .scopeFromProject()
            .files
            .filter { it.packagee?.name?.contains(".feature.") == true }
            .assertTrue {
                val imports = it.imports.map { import -> import.name }
                val packageName = it.packagee?.name ?: ""

                // Check if imports contain other feature packages
                val hasFeatureDependency = imports.any { import ->
                    import.contains(".feature.") && !import.startsWith(packageName)
                }

                !hasFeatureDependency
            }
    }

    @Test
    fun `domain layer should not depend on feature layer`() {
        Konsist
            .scopeFromProject()
            .files
            .filter { it.packagee?.name?.contains(".domain.") == true }
            .assertTrue {
                it.imports.none { import -> import.name.contains(".feature.") }
            }
    }
}
